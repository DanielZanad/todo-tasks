use actix_web::{Error, HttpResponse, error, post, web};
use serde::{Deserialize, Serialize};

use crate::{
    app::use_cases::{
        create_user_session_use_case::{CreateUserSessionRequest, CreateUserSessionUseCase},
        get_signed_url_use_case, register_user_use_case,
    },
    env::get_env_var,
};

#[derive(Deserialize, Serialize, Debug)]
struct Body {
    email: String,
    password: String,
}

#[post("/session")]
pub async fn create_user_session_controller(
    request_body: web::Json<Body>,
    create_user_session_use_case: web::Data<CreateUserSessionUseCase>,
) -> Result<HttpResponse, Error> {
    println!("Logging user");

    let create_user_session_request =
        CreateUserSessionRequest::new(request_body.email.clone(), request_body.password.clone());

    let token = create_user_session_use_case
        .execute(create_user_session_request)
        .await;

    match token {
        Ok(token) => Ok(HttpResponse::Ok().json(serde_json::json!({ "token":  token.token}))),
        Err(e) => Err(error::ErrorInternalServerError(e.to_string())),
    }
}
