use std::collections::HashSet;

use actix_web::{
    App, Error, HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error,
    http::header,
    middleware::{Next, from_fn},
};
use jsonwebtoken::{DecodingKey, Header, decode};
use serde::{Deserialize, Serialize};

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

/// A struct to hold the authenticated user's ID.
/// We'll insert this into request extensions.
#[derive(Clone)]
pub struct AuthenticatedUser {
    pub id: String,
}

pub async fn check_request_jwt(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // Attempt to get the "Authorization" header from the request.
    let header = req.headers().get(header::AUTHORIZATION);

    // --- Pre-processing ---
    // This part runs before the request is passed to the handler.

    match header {
        Some(header) => {
            let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

            let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS512);

            validation.required_spec_claims = HashSet::new();
            validation.validate_aud = false;

            let decode_token = decode::<Claims>(
                header.to_str().unwrap_or(""),
                &DecodingKey::from_secret(jwt_secret.as_ref()),
                &validation,
            );

            match decode_token {
                Ok(token_data) => {
                    req.extensions_mut().insert(AuthenticatedUser {
                        id: token_data.claims.sub,
                    });
                }
                Err(e) => {
                    eprintln!("JWT Decode Error: {:?}", e);
                    panic!("Failed to decode token: {}", e);
                }
            }
        }
        None => {
            println!("Blocking request: Missing Authorization header.");
            return Err(error::ErrorUnauthorized(
                "Authorization header is required.",
            ));
        }
    }

    println!("Request passed middleware pre-processing.");

    let res = next.call(req).await?;

    println!("Middleware post-processing the response.");

    Ok(res)
}
