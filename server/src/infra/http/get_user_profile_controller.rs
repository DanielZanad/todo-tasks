use actix_web::{Error, HttpResponse, error, get, web};

use crate::{
    app::use_cases::get_user_profile_use_case::{GetUserProfileRequest, GetUserProfileUseCase},
    infra::middlewares::check_request_jwt::AuthenticatedUser,
};

#[get("/profile")]
pub async fn get_user_profile_controller(
    user: web::ReqData<AuthenticatedUser>,
    get_user_profile_use_case: web::Data<GetUserProfileUseCase>,
) -> Result<HttpResponse, Error> {
    println!("Get user profile");

    let get_user_profile_request = GetUserProfileRequest::new(user.id.clone());

    let response = get_user_profile_use_case
        .execute(get_user_profile_request)
        .await;

    match response {
        Ok(user) => Ok(HttpResponse::Ok().json(serde_json::json!({ "username":  user.user_profile.user.username(), "email":  user.user_profile.user.email(), "avatar_url":  user.user_profile.avatar_url    }))),
        Err(e) => Err(error::ErrorInternalServerError(e.to_string())),
    }
}
