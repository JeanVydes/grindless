use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Errors {
    NotFound,
    BadRequest,
    InternalServerError,
    Unauthorized,
    Forbidden,
    Conflict,
    UnprocessableEntity,
    TooManyRequests,
    ServiceUnavailable,
    GatewayTimeout,

    InvalidToken,
    ExpiredToken,
}

impl Errors {
    pub fn to_string(&self) -> String {
        match self {
            Errors::NotFound => "Not Found".to_string(),
            Errors::BadRequest => "Bad Request".to_string(),
            Errors::InternalServerError => "Internal Server Error".to_string(),
            Errors::Unauthorized => "Unauthorized".to_string(),
            Errors::Forbidden => "Forbidden".to_string(),
            Errors::Conflict => "Conflict".to_string(),
            Errors::UnprocessableEntity => "Unprocessable Entity".to_string(),
            Errors::TooManyRequests => "Too Many Requests".to_string(),
            Errors::ServiceUnavailable => "Service Unavailable".to_string(),
            Errors::GatewayTimeout => "Gateway Timeout".to_string(),
            Errors::InvalidToken => "Invalid Token".to_string(),
            Errors::ExpiredToken => "Expired Token".to_string(),
        }
    }
}