use actix_web::{Error, HttpResponse, error, get, post, web};
use serde::{Deserialize, Serialize};

use crate::{
    app::use_cases::{
        create_user_session_use_case::{CreateUserSessionRequest, CreateUserSessionUseCase},
        get_signed_url_use_case,
        get_user_profile_use_case::{GetUserProfileRequest, GetUserProfileUseCase},
        register_user_use_case,
    },
    env::get_env_var,
    infra::middlewares::check_request_jwt::AuthenticatedUser,
};

#[derive(Deserialize, Serialize, Debug)]
struct Body {
    token: String,
}

#[get("/profile")]
pub async fn get_user_profile_controller(
    user: web::ReqData<AuthenticatedUser>,
    get_user_profile_use_case: web::Data<GetUserProfileUseCase>,
) -> Result<HttpResponse, Error> {
    println!("Get user profile");

    println!("Get user profile for user_id: {}", user.id);

    Ok(HttpResponse::Ok().body("data"))
}
