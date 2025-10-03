use std::pin::Pin;

use crate::app::entities::task::Task;

pub trait TaskRepository {
    fn save<'a>(&'a self, task: Task) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;
}
