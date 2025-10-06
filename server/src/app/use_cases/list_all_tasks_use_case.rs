use std::sync::Arc;

use actix_web::rt::task;
use serde::{Deserialize, Serialize};

use crate::app::{entities::task::Task, repositories::task_repository::TaskRepository};

#[derive(Serialize, Deserialize)]
pub struct ListAllTasksResponse {
    pub tasks: Vec<Task>,
}

impl ListAllTasksResponse {
    pub fn new(tasks: Vec<Task>) -> Self {
        Self { tasks }
    }
}

pub struct ListAllTasksRequest {
    user_id: String,
}

impl ListAllTasksRequest {
    pub fn new(user_id: String) -> Self {
        Self { user_id }
    }
}

pub struct ListAllTasksUseCase {
    task_repository: Arc<dyn TaskRepository + Send + Sync>,
}

impl ListAllTasksUseCase {
    pub fn new(task_repository: Arc<dyn TaskRepository + Send + Sync>) -> Self {
        Self { task_repository }
    }

    pub async fn execute(&self, request: ListAllTasksRequest) -> ListAllTasksResponse {
        let tasks =
            ListAllTasksResponse::new(self.task_repository.list_all_tasks(request.user_id).await);
        tasks
    }
}
