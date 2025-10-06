use std::sync::Arc;

use chrono::{Date, DateTime, Local};

use crate::app::{entities::task::Task, repositories::task_repository::TaskRepository};

pub struct SaveTaskRequest {
    pub user_id: String,
    pub task_date: DateTime<Local>,
    pub content: String,
}

impl SaveTaskRequest {
    pub fn new(user_id: String, content: String, task_date: DateTime<Local>) -> Self {
        Self {
            user_id,
            content,
            task_date,
        }
    }
}

pub struct SaveTaskUseCase {
    task_repository: Arc<dyn TaskRepository + Send + Sync>,
}

impl SaveTaskUseCase {
    pub fn new(task_repository: Arc<dyn TaskRepository + Send + Sync>) -> Self {
        Self { task_repository }
    }

    pub async fn execute(&self, request: SaveTaskRequest) {
        let task = Task::new(request.user_id, request.content, request.task_date);

        self.task_repository.save(task).await;
    }
}
