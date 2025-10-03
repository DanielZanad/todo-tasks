use std::sync::Arc;

use crate::app::{entities::task::Task, repositories::task_repository::TaskRepository};

pub struct SaveTaskRequest {
    pub user_id: String,
    pub content: String,
}

impl SaveTaskRequest {
    pub fn new(user_id: String, content: String) -> Self {
        Self { user_id, content }
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
        let task = Task::new(request.user_id, request.content);

        self.task_repository.save(task).await;
    }
}
