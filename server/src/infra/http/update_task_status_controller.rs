use actix_web::{Error, HttpResponse, error, get, put, web};

use crate::{
    app::use_cases::update_task_status_use_case::{
        UpdateTaskStatusRequest, UpdateTaskStatusUseCase,
    },
    infra::middlewares::check_request_jwt::AuthenticatedUser,
};

#[put("/update/{task_id}/{action}")]
pub async fn update_task_status_controller(
    user: web::ReqData<AuthenticatedUser>,
    path: web::Path<(String, String)>,
    update_task_status_use_case: web::Data<UpdateTaskStatusUseCase>,
) -> Result<HttpResponse, Error> {
    println!("Update task controller: =====================================");
    let update_task_status_request =
        UpdateTaskStatusRequest::new(user.id.clone(), path.0.clone(), path.1.clone());

    let response = update_task_status_use_case
        .execute(update_task_status_request)
        .await;

    if let Err(e) = response {
        eprintln!("{}", e);
        return Err(error::ErrorInternalServerError("Failed to process request"));
    }

    Ok(HttpResponse::Ok().into())
}
