use actix_web::{Error, HttpResponse, get, post, rt::task, web};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::{
    app::use_cases::{
        list_all_tasks_use_case::{ListAllTasksRequest, ListAllTasksUseCase},
        save_task_use_case::{SaveTaskRequest, SaveTaskUseCase},
    },
    infra::middlewares::check_request_jwt::AuthenticatedUser,
};

#[get("/list")]
pub async fn list_all_user_tasks_controller(
    user: web::ReqData<AuthenticatedUser>,
    list_all_tasks_use_case: web::Data<ListAllTasksUseCase>,
) -> Result<HttpResponse, Error> {
    println!("List controller: =====================================");
    let list_all_user_tasks_request = ListAllTasksRequest::new(user.id.clone());

    let response = list_all_tasks_use_case
        .execute(list_all_user_tasks_request)
        .await;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "tasks":  response.tasks  })))
}
