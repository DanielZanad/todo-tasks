use std::sync::Arc;

use actix_web::error;
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core},
};
use serde::{Deserialize, Serialize};

use crate::{
    app::{entities::user::User, repositories::user_repository::UserRepository},
    env::get_env_var,
};

pub struct RegisterUserRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    pub file_key: Option<String>,
    pub mime_type: Option<String>,
}

impl RegisterUserRequest {
    pub fn new(
        email: String,
        username: String,
        password: String,
        file_key: Option<String>,
        mime_type: Option<String>,
    ) -> Self {
        Self {
            email,
            username,
            password,
            file_key,
            mime_type,
        }
    }
}

#[derive(Debug)]
pub struct RegisterUserResponse {
    pub url: String,
}

impl RegisterUserResponse {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

#[derive(Serialize)]
struct OutgoingPayload {
    file_key: String,
    content_type: String,
}

impl OutgoingPayload {
    pub fn new(file_key: String, content_type: String) -> Self {
        Self {
            file_key,
            content_type,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct IncomingPayload {
    url: String,
}

pub struct RegisterUserUseCase {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
}

impl RegisterUserUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, request: RegisterUserRequest) -> RegisterUserResponse {
        let mut user = User::new(request.email, request.username, request.password);
        let salt = SaltString::generate(rand_core::OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(user.password().as_bytes(), &salt)
            .unwrap()
            .to_string();

        user.set_password(password_hash);

        let mut file_key = request
            .file_key
            .clone()
            .unwrap_or_else(|| get_env_var("DEFAULT_AVATAR").expect("DEFAULT_AVATAR must be set"));

        let mime_type = request
            .mime_type
            .clone()
            .unwrap_or_else(|| String::from("image/jpeg"));

        file_key = format!("{file_key}-{}", uuid::Uuid::new_v4());

        let api_url = get_env_var("SIGNED_URL_API").expect("SIGNED_URL_API must be set");
        let client = reqwest::Client::new();
        let payload = OutgoingPayload::new(file_key.clone(), mime_type.clone());
        let payload = serde_json::json!({
            "fileKey": payload.file_key.clone(),
            "contentType": payload.content_type
        });

        let result = client
            .post(format!("{}/uploads", api_url))
            .json(&payload)
            .send()
            .await
            .map_err(|_| error::ErrorBadRequest("Error when getting the signed url"))
            .map_err(|e| eprint!("Error when getting the signed url: {}", e))
            .unwrap();

        let signed_url: IncomingPayload = result
            .json()
            .await
            .map_err(|_| error::ErrorBadRequest("Invalid response from the signed api"))
            .map_err(|e| eprintln!("Invalid response from the signed api: {}", e))
            .unwrap();

        self.user_repository
            .register(user, file_key, mime_type)
            .await;

        return RegisterUserResponse::new(signed_url.url);
    }
}
