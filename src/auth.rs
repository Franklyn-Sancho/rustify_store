use actix_web::{dev::Payload, error, web, HttpRequest, HttpResponse, FromRequest};
use futures::future::{ready, Ready};
use actix_web::Error;

use crate::jwt::{validate_token, Claims};

pub struct AuthenticatedUser(pub Claims);

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];
                    match validate_token(token) {
                        Ok(claims) => return ready(Ok(AuthenticatedUser(claims))),
                        Err(err) => {
                            eprintln!("Token validation failed: {:?}", err);
                            return ready(Err(actix_web::error::ErrorUnauthorized("Unauthorized")));
                        }
                    }
                }
            }
        }
        eprintln!("Authorization header missing or invalid");
        ready(Err(actix_web::error::ErrorUnauthorized("Unauthorized")))
    }
}


