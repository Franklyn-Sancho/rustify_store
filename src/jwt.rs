use std::{env, sync::Arc};

use actix_web::error;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse},
    Error,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use actix_service::Transform;

use futures::future::{ok, Either, Ready};

use std::task::{Context, Poll};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,  // User ID
    pub exp: usize, // Expiration timestamp
}

/// Creates a JWT for the given user ID.
pub fn create_jwt(user_id: Uuid) -> Result<String, Box<dyn std::error::Error>> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(3600))
        .expect("Invalid expiration time")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");
    let encoding_key = EncodingKey::from_secret(secret.as_ref());
    let token = encode(&Header::default(), &claims, &encoding_key)?;

    Ok(token)
}

/// Middleware for JWT authentication
pub struct JwtMiddleware<S> {
    service: S,
}

impl<S, Req> Transform<S, ServiceRequest> for JwtMiddleware<Req>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddleware { service })
    }
}

impl<S> Service<ServiceRequest> for JwtMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Extract token from the Authorization header
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(token) = auth_header.to_str() {
                match validate_token(token) {
                    Ok(_) => {
                        // Token is valid, proceed to the next service
                        return Either::Left(self.service.call(req));
                    }
                    Err(_) => {
                        return Either::Right(ok(
                            req.error_response(error::ErrorUnauthorized("Invalid token"))
                        ))
                    }
                }
            }
        }

        Either::Right(ok(req.error_response(error::ErrorUnauthorized(
            "Authorization token missing",
        ))))
    }
}

// JWT token validation function
pub fn validate_token(token: &str) -> Result<Claims, Box<dyn std::error::Error>> {
    println!("Validating token: {}", token);
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not set");
    let decoding_key = jsonwebtoken::DecodingKey::from_secret(secret.as_ref());
    let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);

    let token_data = jsonwebtoken::decode::<Claims>(token, &decoding_key, &validation)?;
    Ok(token_data.claims)
}
