use actix_web::web;
use grindless_core::{
    response::response::{build_err, ResponseBuilderError, ResponseObject},
    util::jwt::{self, JWTClaims},
};
use jsonwebtoken::Algorithm;
use serde::Serialize;

pub fn get_claims_from_header<T>(
    authorization_header: Option<&actix_web::http::header::HeaderValue>,
    public_pem: Vec<u8>,
) -> Result<JWTClaims, web::Json<ResponseObject<T>>>
where
    T: Serialize,
{
    let authorization_header = match authorization_header {
        Some(h) => h,
        None => {
            return Err(build_err::<T>(ResponseBuilderError {
                message: "No authorization header".to_string(),
                errors: vec![],
            }))
        }
    };

    let authorization_header = match authorization_header.to_str() {
        Ok(a) => a.to_string(),
        Err(_) => {
            return Err(build_err::<T>(ResponseBuilderError {
                message: "Error parsing authorization header".to_string(),
                errors: vec![],
            }))
        }
    };

    let mut parts = authorization_header.split_whitespace();
    match parts.next() {
        Some(bearer) => {
            if bearer != "Bearer" {
                return Err(build_err(ResponseBuilderError {
                    message: "Invalid authorization header".to_string(),
                    errors: vec![],
                }));
            }
        }
        None => {
            return Err(build_err(ResponseBuilderError {
                message: "Invalid authorization header".to_string(),
                errors: vec![],
            }));
        }
    }

    let token = match parts.next() {
        Some(token) => token.to_string(),
        None => {
            return Err(build_err(ResponseBuilderError {
                message: "Invalid authorization header".to_string(),
                errors: vec![],
            }));
        }
    };

    let validate_token = match jwt::validate_token(&public_pem, Algorithm::RS256, &token) {
        Ok(t) => t,
        Err(_) => {
            return Err(build_err(ResponseBuilderError {
                message: "Invalid token".to_string(),
                errors: vec![],
            }))
        }
    };

    Ok(validate_token)
}
