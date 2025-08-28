use actix_web::{Error, HttpResponse, error, post, web};
use serde::{Deserialize, Serialize};

use crate::{
    app::use_cases::{
        create_user_session_use_case::{CreateUserSessionRequest, CreateUserSessionUseCase},
        get_signed_url_use_case,
        get_user_profile_use_case::{GetUserProfileRequest, GetUserProfileUseCase},
        register_user_use_case,
    },
    env::get_env_var,
};

#[derive(Deserialize, Serialize, Debug)]
struct Body {
    token: String,
}

#[post("/profile")]
pub async fn get_user_profile_controller(
    request_body: web::Json<Body>,
    get_user_profile_use_case: web::Data<GetUserProfileUseCase>,
) -> Result<HttpResponse, Error> {
    println!("Get user profile");

    let create_user_session_request = GetUserProfileRequest::new(request_body.token.clone());

    let token = get_user_profile_use_case
        .execute(create_user_session_request)
        .await;

    Ok(HttpResponse::Ok().body("data"))
}
