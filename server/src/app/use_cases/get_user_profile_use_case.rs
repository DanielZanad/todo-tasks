use core::fmt;
use std::{collections::HashSet, sync::Arc};

use argon2::{
    Argon2, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core},
};
use chrono::Utc;
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Error,
};
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
    pub token: String,
}

impl GetUserProfileRequest {
    pub fn new(token: String) -> Self {
        Self { token }
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
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS512);

        validation.required_spec_claims = HashSet::new();
        validation.validate_aud = false;

        let decode_token = decode::<Claims>(
            &request.token,
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &validation,
        );

        match decode_token {
            Ok(token_data) => {
                println!("Decoded claims: {:?}", token_data.claims);

                let user = User::new(
                    String::from("email"),
                    String::from("username"),
                    String::from("password"),
                );
                let response = GetUserProfileResponse::new(user);
                Ok(response)
            }
            Err(e) => {
                eprintln!("JWT Decode Error: {:?}", e);
                panic!("Failed to decode token: {}", e);
            }
        }
    }
}
