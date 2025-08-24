use std::{future::Future, pin::Pin};

use crate::app::entities::user::User;

pub trait UserRepository {
    fn register<'a>(
        &'a self,
        user: User,
        file_key: String,
        mime_type: String,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;
    fn get_user_by_email<'a>(
        &'a self,
        email: String,
    ) -> Pin<Box<dyn Future<Output = Option<User>> + Send + 'a>>;
}
