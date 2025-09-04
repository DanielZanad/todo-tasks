use std::{future::Future, pin::Pin};

use crate::app::entities::user::User;

pub struct UserProfile {
    pub user: User,
    pub avatar_url: String,
}

impl UserProfile {
    pub fn set_avatar_url(&mut self, avatar_url: String) {
        self.avatar_url = avatar_url;
    }
}

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
    fn get_user_profile<'a>(
        &'a self,
        user_id: String,
    ) -> Pin<Box<dyn Future<Output = Option<UserProfile>> + Send>>;
}
