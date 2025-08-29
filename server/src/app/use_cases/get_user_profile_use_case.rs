use core::fmt;
use std::sync::Arc;

use jsonwebtoken::Header;
use serde::{Deserialize, Serialize};

use crate::app::{entities::user::User, repositories::user_repository::UserRepository};

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Debug, Deserialize, Serialize)]
struct Token {
    claims: Claims,
    header: Header,
}

impl Token {
    pub fn new(claims: Claims, header: Header) -> Self {
        Self { claims, header }
    }
}

pub struct GetUserProfileRequest {
    pub user_id: String,
}

impl GetUserProfileRequest {
    pub fn new(user_id: String) -> Self {
        Self { user_id }
    }
}

pub struct GetUserProfileResponse {
    pub user: User,
}

impl GetUserProfileResponse {
    pub fn new(user: User) -> Self {
        Self { user }
    }
}

pub enum GetUserProfileError {
    UserNotFound(String),
}

impl fmt::Display for GetUserProfileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetUserProfileError::UserNotFound(e) => {
                write!(f, "User not found in database: {}", e)
            }
        }
    }
}

pub struct GetUserProfileUseCase {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
}

impl GetUserProfileUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(
        &self,
        request: GetUserProfileRequest,
    ) -> Result<GetUserProfileResponse, GetUserProfileError> {
        let user = User::new(
            String::from("email"),
            String::from("username"),
            String::from("password"),
        );
        let response = GetUserProfileResponse::new(user);
        Ok(response)
    }
}
