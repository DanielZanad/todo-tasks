use actix_web::{Error, HttpResponse, post, web};
use serde::{Deserialize, Serialize};

use crate::app::use_cases::register_user_use_case;

#[derive(Deserialize, Serialize, Debug)]
struct User {
    email: String,
    username: String,
    password: String,
    file_key: Option<String>,
    mime_type: Option<String>,
}

#[post("/users")]
pub async fn register_user_route(
    user: web::Json<User>,
    register_user: web::Data<register_user_use_case::RegisterUserUseCase>,
) -> Result<HttpResponse, Error> {
    println!("Registering user");

    let request = register_user_use_case::RegisterUserRequest::new(
        user.email.to_owned(),
        user.username.to_owned(),
        user.password.to_owned(),
        user.file_key.to_owned(),
        user.mime_type.to_owned(),
    );

    let response = register_user.execute(request).await;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "url": response.url })))
}
