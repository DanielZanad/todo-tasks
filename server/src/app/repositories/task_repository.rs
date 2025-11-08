use std::pin::Pin;

use crate::app::entities::{task::Task, task_status::TaskStatus};

pub trait TaskRepository {
    fn save<'a>(&'a self, task: Task) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;
    fn list_all<'a>(
        &'a self,
        user_id: String,
    ) -> Pin<Box<dyn Future<Output = Vec<Task>> + Send + 'a>>;
    fn find_by_id<'a>(
        &'a self,
        user_id: String,
    ) -> Pin<Box<dyn Future<Output = Option<Task>> + Send + 'a>>;
    fn update_status<'a>(
        &'a self,
        user_id: String,
        task_id: String,
        status: TaskStatus,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;
}
