use actix_web::{Error, HttpResponse, error, post, web};
use serde::{Deserialize, Serialize};

use crate::{
    app::use_cases::{
        get_signed_url_use_case::{self, GetSignedUrlUseCase},
        register_user_use_case::{self, RegisterUserUseCase},
    },
    env::get_env_var,
};

#[derive(Deserialize, Serialize, Debug)]
struct Body {
    email: String,
    username: String,
    password: String,
    file_key: Option<String>,
    mime_type: Option<String>,
}

#[post("/users")]
pub async fn register_user_controller(
    request_body: web::Json<Body>,
    register_user_use_case: web::Data<RegisterUserUseCase>,
    get_signed_url_use_case: web::Data<GetSignedUrlUseCase>,
) -> Result<HttpResponse, Error> {
    println!("Registering user");

    let mut file_key = request_body
        .file_key
        .clone()
        .unwrap_or_else(|| get_env_var("DEFAULT_AVATAR").expect("DEFAULT_AVATAR must be set"));

    let mime_type = request_body
        .mime_type
        .clone()
        .unwrap_or_else(|| String::from("image/jpeg"));

    file_key = format!("{file_key}-{}", uuid::Uuid::new_v4());

    let register_user_request = register_user_use_case::RegisterUserRequest::new(
        request_body.email.to_owned(),
        request_body.username.to_owned(),
        request_body.password.to_owned(),
        file_key.clone(),
        mime_type.clone(),
    );

    let get_signed_url_request = get_signed_url_use_case::GetSignedUrlUseCaseRequest::new(
        file_key.clone(),
        mime_type.clone(),
    );

    register_user_use_case.execute(register_user_request).await;
    let response = get_signed_url_use_case
        .execute(get_signed_url_request)
        .await
        .map_err(|e| {
            eprintln!("Failed to get signed URL: {}", e);
            error::ErrorInternalServerError("Failed to process request")
        })?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "url":  response.url})))
}
