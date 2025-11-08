use std::fmt;
use std::sync::Arc;

use crate::app::entities::task_status::TaskStatus::*;
use crate::app::{entities::task::Task, repositories::task_repository::TaskRepository};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ListAllTasksResponse {
    pub tasks: Vec<Task>,
}

impl ListAllTasksResponse {
    pub fn new(tasks: Vec<Task>) -> Self {
        Self { tasks }
    }
}

pub struct UpdateTaskStatusRequest {
    user_id: String,
    task_id: String,
    action: String,
}

impl UpdateTaskStatusRequest {
    pub fn new(user_id: String, task_id: String, action: String) -> Self {
        Self {
            user_id,
            action,
            task_id,
        }
    }
}

pub enum UpdateTaskStatusError {
    TaskNotFound(),
    RequestFailed(reqwest::Error),
}

impl fmt::Display for UpdateTaskStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpdateTaskStatusError::TaskNotFound() => {
                write!(f, "Task not found in database")
            }
            UpdateTaskStatusError::RequestFailed(e) => {
                write!(f, "Request to signed URL API failed: {}", e)
            }
        }
    }
}

pub struct UpdateTaskStatusUseCase {
    task_repository: Arc<dyn TaskRepository + Send + Sync>,
}

impl UpdateTaskStatusUseCase {
    pub fn new(task_repository: Arc<dyn TaskRepository + Send + Sync>) -> Self {
        Self { task_repository }
    }

    pub async fn execute(
        &self,
        request: UpdateTaskStatusRequest,
    ) -> Result<(), UpdateTaskStatusError> {
        let task = self
            .task_repository
            .find_by_id(request.task_id.clone())
            .await;

        if task.is_none() {
            return Err(UpdateTaskStatusError::TaskNotFound());
        }
        let task = task.unwrap();
        if task.user_id() != request.user_id {
            return Err(UpdateTaskStatusError::TaskNotFound());
        }
        let new_status = {
            if *task.status() == Completed && request.action == "next" {
                return Ok(());
            }
            if *task.status() == ToStart && request.action == "previous" {
                return Ok(());
            }
            if *task.status() == ToStart && request.action == "next" {
                Started
            } else if *task.status() == Started && request.action == "next" {
                Completed
            } else if *task.status() == Started && request.action == "previous" {
                ToStart
            } else if *task.status() == Completed && request.action == "previous" {
                Started
            } else {
                return Ok(());
            }
        };

        self.task_repository
            .update_status(request.user_id, request.task_id, new_status)
            .await;
        Ok(())
    }
}
