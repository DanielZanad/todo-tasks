use std::any::Any;

use actix_web::{Error, HttpResponse, error, post, web};
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

#[derive(Deserialize, Serialize, Debug)]
struct User {
    email: String,
    username: String,
    password: String,
    file_key: Option<String>,
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

    file_key = format!("{file_key}-{}", uuid::Uuid::new_v4());

    let api_url = get_env_var("SIGNED_URL_API").expect("SIGNED_URL_API must be set");
    let client = reqwest::Client::new();
    let payload = OutgoingPayload::new(file_key.clone(), String::from("image/jpeg"));
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
        user.password,
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(|e| {
        eprintln!("Failed to create user: {}", e);
        error::ErrorInternalServerError("Failed to register user.")
    })?;

    let user_id = new_user.id;

    sqlx::query!(
        r#"
        INSERT INTO avatars (user_id, file_key)
        VALUES ($1, $2)
        "#,
        user_id,
        file_key
    )
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

    Ok(HttpResponse::Ok().json(signed_url))
}
