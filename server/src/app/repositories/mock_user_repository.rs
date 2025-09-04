use crate::app::entities::avatar;
use crate::app::entities::{avatar::Avatar, user::User};
use crate::app::repositories::user_repository::{UserProfile, UserRepository};
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core},
};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub struct MockUserRepository {
    pub users: Arc<Mutex<Vec<User>>>,
    pub avatars: Arc<Mutex<Vec<Avatar>>>,
}

impl MockUserRepository {
    pub fn new() -> Self {
        Self::default()
    }

    // Helper for tests to pre-populate data for login tests.
    // Hashes password and sets ID.
    pub fn add_user(&self, mut user: User) {
        let salt = SaltString::generate(rand_core::OsRng);
        let password_hash = Argon2::default()
            .hash_password(user.password().as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string();
        user.set_password(password_hash);
        if user.id().is_none() {
            user.set_id(uuid::Uuid::new_v4().to_string());
        }
        let avatar = Avatar::new(
            uuid::Uuid::new_v4().to_string(),
            user.id().unwrap().clone(),
            String::from("file_key"),
            String::from("mime_type"),
        );

        self.avatars.lock().unwrap().push(avatar);
        self.users.lock().unwrap().push(user);
    }

    // Helper for tests to inspect the state
    pub fn find_user_by_email(&self, email: &str) -> Option<User> {
        self.users
            .lock()
            .unwrap()
            .iter()
            .find(|u| u.email() == email)
            .cloned()
    }
}

impl UserRepository for MockUserRepository {
    fn register<'a>(
        &'a self,
        mut user: User,
        _file_key: String,
        _mime_type: String,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        // The use case should have already hashed the password.
        // The repository is responsible for assigning an ID.
        if user.id().is_none() {
            user.set_id(uuid::Uuid::new_v4().to_string());
        }
        self.users.lock().unwrap().push(user);
        Box::pin(async {})
    }

    fn get_user_by_email<'a>(
        &'a self,
        email: String,
    ) -> Pin<Box<dyn Future<Output = Option<User>> + Send + 'a>> {
        let users = self.users.lock().unwrap();
        let user = users.iter().find(|u| u.email() == email).cloned();
        Box::pin(async move { user })
    }

    fn get_user_profile<'a>(
        &'a self,
        user_id: String,
    ) -> Pin<Box<dyn Future<Output = Option<UserProfile>> + Send>> {
        let users = self.users.lock().unwrap();
        let user = users.iter().find(|u| u.id() == Some(&user_id)).cloned();

        match user {
            Some(user) => {
                let avatars = self.avatars.lock().unwrap();
                let avatar = avatars.iter().find(|a| a.user_id() == user_id).cloned();
                match avatar {
                    Some(avatar) => {
                        let user_profile = UserProfile {
                            user,
                            avatar_url: avatar.file_key().to_string(),
                        };
                        Box::pin(async move { Some(user_profile) })
                    }
                    None => Box::pin(async move { None }),
                }
            }
            None => Box::pin(async move { None }),
        }
    }
}
