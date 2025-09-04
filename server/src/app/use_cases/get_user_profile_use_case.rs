use core::fmt;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{
    app::{
        entities::user::{self, User},
        repositories::user_repository::{UserProfile, UserRepository},
    },
    env::get_env_var,
};

pub struct GetUserProfileRequest {
    pub user_id: String,
}

impl GetUserProfileRequest {
    pub fn new(user_id: String) -> Self {
        Self { user_id }
    }
}

pub struct GetUserProfileResponse {
    pub user_profile: UserProfile,
}

impl GetUserProfileResponse {
    pub fn new(user_profile: UserProfile) -> Self {
        Self { user_profile }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IncomingPayload {
    pub url: String,
}

#[derive(Serialize)]
pub struct OutgoingPayload {
    file_key: String,
}

impl OutgoingPayload {
    pub fn new(file_key: String) -> Self {
        Self { file_key }
    }
}

pub enum GetUserProfileError {
    UserNotFound(String),
    RequestFailed(reqwest::Error),
    InvalidResponse(reqwest::Error),
    ApiError {
        status: reqwest::StatusCode,
        body: String,
    },
    MissingEnvVar(String),
}

impl fmt::Display for GetUserProfileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetUserProfileError::UserNotFound(e) => {
                write!(f, "User not found in database: {}", e)
            }
            GetUserProfileError::RequestFailed(e) => {
                write!(f, "Request to signed URL API failed: {}", e)
            }
            GetUserProfileError::InvalidResponse(e) => {
                write!(f, "Invalid response from signed URL API: {}", e)
            }
            GetUserProfileError::ApiError { status, body } => {
                write!(f, "Signed URL API returned error: {} - {}", status, body)
            }
            GetUserProfileError::MissingEnvVar(e) => {
                write!(f, "Missing environment variable: {}", e)
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
        let user_profile = self.user_repository.get_user_profile(request.user_id).await;
        let api_url = get_env_var("SIGNED_URL_API").ok_or_else(|| {
            GetUserProfileError::MissingEnvVar("SIGNED_URL_API must be set".to_string())
        })?;
        let client = reqwest::Client::new();

        match user_profile {
            Some(mut user_profile) => {
                let payload = OutgoingPayload::new(user_profile.avatar_url.clone());

                let result = client
                    .get(format!("{}/uploads/{}", api_url, user_profile.avatar_url))
                    .send()
                    .await
                    .map_err(GetUserProfileError::RequestFailed)?;

                if !result.status().is_success() {
                    let status = result.status();
                    let body = result
                        .text()
                        .await
                        .unwrap_or_else(|_| "Could not retrieve error body".to_string());
                    return Err(GetUserProfileError::ApiError { status, body });
                }

                let avatar_url: IncomingPayload = result
                    .json()
                    .await
                    .map_err(GetUserProfileError::InvalidResponse)?;
                println!("Avatar URL: {:?}", avatar_url);
                user_profile.set_avatar_url(avatar_url.url);
                Ok(GetUserProfileResponse::new(user_profile))
            }
            None => Err(GetUserProfileError::UserNotFound(
                "User not found".to_string(),
            )),
        }
    }
}
