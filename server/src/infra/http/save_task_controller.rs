use actix_web::{Error, HttpResponse, post, web};
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    app::use_cases::save_task_use_case::{SaveTaskRequest, SaveTaskUseCase},
    infra::middlewares::check_request_jwt::AuthenticatedUser,
};

#[derive(Deserialize, Serialize, Debug)]
struct Body {
    content: String,
    task_date: String,
}

#[post("/save")]
pub async fn save_task_controller(
    user: web::ReqData<AuthenticatedUser>,
    request_body: web::Json<Body>,
    save_task_use_case: web::Data<SaveTaskUseCase>,
) -> Result<HttpResponse, Error> {
    let task_date: DateTime<Utc> = request_body
        .task_date
        .parse()
        .expect("Failed to parse date");

    println!("{:?}", task_date);

    let save_task_use_case_request =
        SaveTaskRequest::new(user.id.clone(), request_body.content.to_owned(), task_date);

    save_task_use_case.execute(save_task_use_case_request).await;

    Ok(HttpResponse::Ok().into())
}
