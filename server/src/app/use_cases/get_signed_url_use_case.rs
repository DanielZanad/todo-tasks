use std::fmt;

use serde::{Deserialize, Serialize};

use crate::env::get_env_var;

pub struct GetSignedUrlUseCaseRequest {
    pub file_key: String,
    pub mime_type: String,
}

impl GetSignedUrlUseCaseRequest {
    pub fn new(file_key: String, mime_type: String) -> Self {
        Self {
            file_key,
            mime_type,
        }
    }
}

#[derive(Debug)]
pub struct GetSignedUrlUseCaseResponse {
    pub url: String,
}

impl GetSignedUrlUseCaseResponse {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

#[derive(Debug)]
pub enum GetSignedUrlError {
    RequestFailed(reqwest::Error),
    InvalidResponse(reqwest::Error),
    ApiError {
        status: reqwest::StatusCode,
        body: String,
    },
    MissingEnvVar(String),
}

impl fmt::Display for GetSignedUrlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GetSignedUrlError::RequestFailed(e) => {
                write!(f, "Request to signed URL API failed: {}", e)
            }
            GetSignedUrlError::InvalidResponse(e) => {
                write!(f, "Invalid response from signed URL API: {}", e)
            }
            GetSignedUrlError::ApiError { status, body } => {
                write!(f, "Signed URL API returned error: {} - {}", status, body)
            }
            GetSignedUrlError::MissingEnvVar(e) => write!(f, "Missing environment variable: {}", e),
        }
    }
}

impl std::error::Error for GetSignedUrlError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            GetSignedUrlError::RequestFailed(e) | GetSignedUrlError::InvalidResponse(e) => Some(e),
            _ => None,
        }
    }
}

#[derive(Serialize)]
pub struct OutgoingPayload {
    file_key: String,
    content_type: String,
}

impl OutgoingPayload {
    pub fn new(file_key: String, content_type: String) -> Self {
        Self {
            file_key,
            content_type,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IncomingPayload {
    pub url: String,
}

pub struct GetSignedUrlUseCase {}

impl GetSignedUrlUseCase {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn execute(
        &self,
        request: GetSignedUrlUseCaseRequest,
    ) -> Result<GetSignedUrlUseCaseResponse, GetSignedUrlError> {
        let api_url = get_env_var("SIGNED_URL_API").ok_or_else(|| {
            GetSignedUrlError::MissingEnvVar("SIGNED_URL_API must be set".to_string())
        })?;
        println!("API URL, {:?}", api_url);
        let client = reqwest::Client::new();
        let payload = OutgoingPayload::new(request.file_key, request.mime_type);
        let payload = serde_json::json!({
            "fileKey": payload.file_key.clone(),
            "contentType": payload.content_type
        });

        let result = client
            .post(format!("{}/uploads", api_url))
            .json(&payload)
            .send()
            .await
            .map_err(GetSignedUrlError::RequestFailed)?;

        if !result.status().is_success() {
            let status = result.status();
            let body = result
                .text()
                .await
                .unwrap_or_else(|_| "Could not retrieve error body".to_string());
            return Err(GetSignedUrlError::ApiError { status, body });
        }

        let signed_url: IncomingPayload = result
            .json()
            .await
            .map_err(GetSignedUrlError::InvalidResponse)?;

        let response = GetSignedUrlUseCaseResponse::new(signed_url.url);
        Ok(response)
    }
}
