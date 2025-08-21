use std::any::Any;

use actix_web::{Error, HttpResponse, error, post, web};
use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core},
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, types::uuid};

use crate::{env::get_env_var, startup::AppState};

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

#[derive(Serialize, Deserialize)]
struct RegisterUserResponse {
    url: String,
}

impl RegisterUserResponse {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct User {
    email: String,
    username: String,
    password: String,
    file_key: Option<String>,
    mime_type: Option<String>,
}

#[post("/users")]
pub async fn register_user(
    user: web::Json<User>,
    app_data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    println!("Registering user");

    let mut file_key = user
        .file_key
        .clone()
        .unwrap_or_else(|| get_env_var("DEFAULT_AVATAR").expect("DEFAULT_AVATAR must be set"));

    let mime_type = user
        .mime_type
        .clone()
        .unwrap_or_else(|| String::from("image/jpeg"));

    let salt = SaltString::generate(rand_core::OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(user.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    // let hash = PasswordHash::new(&password_hash).unwrap();
    // let password_is_correct = Argon2::default()
    //     .verify_password(user.password.as_bytes(), &hash)
    //     .is_ok();

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
        .map_err(|_| error::ErrorBadRequest("Error when getting the signed url"))?;

    let signed_url: IncomingPayload = result
        .json()
        .await
        .map_err(|_| error::ErrorBadRequest("Invalid response from the signed api"))?;

    // --- Start a database transaction ---
    let mut transaction =
        app_data.conn.begin().await.map_err(|_| {
            error::ErrorInternalServerError("Failed to start database transaction.")
        })?;

    let new_user = sqlx::query!(
        "INSERT INTO users (email, username, password_hash) VALUES ($1, $2, $3) RETURNING id",
        user.email,
        user.username,
        password_hash,
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(|e| {
        eprintln!("Failed to create user: {}", e);
        error::ErrorInternalServerError("Failed to register user.")
    })?;

    let user_id = new_user.id;

    sqlx::query(
        r#"
        INSERT INTO avatars (user_id, file_key, mime_type)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(user_id)
    .bind(file_key)
    .bind(mime_type)
    .execute(&mut *transaction)
    .await
    .map_err(|e| {
        eprintln!("Failed to create avatar: {}", e);
        error::ErrorInternalServerError("Failed to save user avatar.")
    })?;

    transaction
        .commit()
        .await
        .map_err(|_| error::ErrorInternalServerError("Failed to commit database transaction."))?;

    let response: RegisterUserResponse = RegisterUserResponse::new(signed_url.url);

    Ok(HttpResponse::Ok().json(response))
}
