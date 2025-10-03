use actix_web::{Error, HttpResponse, post, web};
use serde::{Deserialize, Serialize};

use crate::{
    app::use_cases::save_task_use_case::{SaveTaskRequest, SaveTaskUseCase},
    infra::middlewares::check_request_jwt::AuthenticatedUser,
};

#[derive(Deserialize, Serialize, Debug)]
struct Body {
    content: String,
}

#[post("/save")]
pub async fn save_task_controller(
    user: web::ReqData<AuthenticatedUser>,
    request_body: web::Json<Body>,
    save_task_use_case: web::Data<SaveTaskUseCase>,
) -> Result<HttpResponse, Error> {
    let save_task_use_case_request =
        SaveTaskRequest::new(user.id.clone(), request_body.content.to_owned());

    save_task_use_case.execute(save_task_use_case_request).await;

    Ok(HttpResponse::Ok().into())
}
