use std::sync::Arc;

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core},
};

use crate::app::{entities::user::User, repositories::user_repository::UserRepository};

pub struct RegisterUserRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    pub file_key: String,
    pub mime_type: String,
}

impl RegisterUserRequest {
    pub fn new(
        email: String,
        username: String,
        password: String,
        file_key: String,
        mime_type: String,
    ) -> Self {
        Self {
            email,
            username,
            password,
            file_key,
            mime_type,
        }
    }
}

pub struct RegisterUserUseCase {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
}

impl RegisterUserUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, request: RegisterUserRequest) {
        let mut user = User::new(request.email, request.username, request.password);
        let salt = SaltString::generate(rand_core::OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(user.password().as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string();

        user.set_password(password_hash);

        self.user_repository
            .register(user, request.file_key, request.mime_type)
            .await;
    }
}
