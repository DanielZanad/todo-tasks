use core::fmt;
use std::sync::Arc;

use argon2::{Argon2, PasswordVerifier};
use chrono::Utc;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode, errors::Error};
use serde::{Deserialize, Serialize};

use crate::app::{entities::user::User, repositories::user_repository::UserRepository};

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub struct CreateUserSessionRequest {
    pub email: String,
    pub password: String,
}

impl CreateUserSessionRequest {
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }
}

pub struct CreateUserSessionResponse {
    pub token: String,
}

impl CreateUserSessionResponse {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

#[derive(Debug)]
pub enum CreateUserSessionError {
    InvalidCredentials(String),
}

impl fmt::Display for CreateUserSessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateUserSessionError::InvalidCredentials(e) => {
                write!(f, "Invalid credentials: {}", e)
            }
        }
    }
}

pub struct CreateUserSessionUseCase {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
}

impl CreateUserSessionUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(
        &self,
        request: CreateUserSessionRequest,
    ) -> Result<CreateUserSessionResponse, CreateUserSessionError> {
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let user = self.user_repository.get_user_by_email(request.email).await;

        if let Some(user) = user {
            let does_password_match = argon2::PasswordHash::new(&user.password())
                .and_then(|hash| {
                    Argon2::default().verify_password(request.password.as_bytes(), &hash)
                })
                .is_ok();

            if !does_password_match {
                return Err(CreateUserSessionError::InvalidCredentials(
                    "Invalid credentials".to_string(),
                ));
            }

            let expiration = Utc::now()
                .checked_add_signed(chrono::Duration::seconds(100000))
                .expect("valid timestamp")
                .timestamp();

            if let Some(user_id) = user.id() {
                let claims = Claims {
                    sub: user_id.to_string(),
                    exp: expiration as usize,
                };
                let header = Header::new(Algorithm::HS512);
                let token = encode(
                    &header,
                    &claims,
                    &EncodingKey::from_secret(jwt_secret.as_ref()),
                )
                .map_err(Error::from);

                if let Ok(token) = token {
                    return Ok(CreateUserSessionResponse::new(token));
                } else {
                    Err(CreateUserSessionError::InvalidCredentials(
                        "Invalid Credential".to_string(),
                    ))
                }
            } else {
                return Err(CreateUserSessionError::InvalidCredentials(
                    "Invalid credentials".to_string(),
                ));
            }
        } else {
            return Err(CreateUserSessionError::InvalidCredentials(
                "Invalid credentials".to_string(),
            ));
        }
    }
}
