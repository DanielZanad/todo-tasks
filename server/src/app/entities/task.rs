use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::app::entities::task_status::TaskStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    id: Option<String>,
    user_id: String,
    content: String,
    status: TaskStatus,
    task_date: DateTime<Local>,
    created_at: DateTime<Local>,
}

impl Task {
    pub fn new(user_id: String, content: String, task_date: DateTime<Local>) -> Self {
        Self {
            id: None,
            user_id,
            content,
            task_date,
            status: TaskStatus::ToStart,
            created_at: Local::now(),
        }
    }

    pub fn new_with_id(
        id: String,
        user_id: String,
        content: String,
        status: TaskStatus,
        task_date: DateTime<Local>,
        created_at: DateTime<Local>,
    ) -> Self {
        Self {
            id: Some(id),
            user_id,
            content,
            status,
            task_date,
            created_at,
        }
    }

    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn status(&self) -> &TaskStatus {
        &self.status
    }

    pub fn created_at(&self) -> &DateTime<Local> {
        &self.created_at
    }

    pub fn task_date(&self) -> &DateTime<Local> {
        &self.task_date
    }

    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    pub fn set_user_id(&mut self, user_id: String) {
        self.user_id = user_id;
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
    }

    pub fn set_task_date(&mut self, task_date: DateTime<Local>) {
        self.task_date = task_date;
    }
}
