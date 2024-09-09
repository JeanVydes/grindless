use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header};
use log::debug;
use serde::{Deserialize, Serialize};

use crate::{response::errors::Errors, Timestamp, ID};

/// # Tokens
///
/// This module contains the token definitions for the apis
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub struct JWTClaims {
    pub r#type: JWTTokenType,
    pub sub: ID,         // Account Id
    pub provider_id: String, // Provider Id
    pub iss: Option<ID>, // Application ID
    pub exp: Timestamp,
    pub max_requests_per_hour: u64,
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum JWTTokenType {
    Access,
    Refresh,
}

impl std::fmt::Display for JWTTokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            JWTTokenType::Access => write!(f, "access"),
            JWTTokenType::Refresh => write!(f, "refresh"),
        }
    }
}

/// # New Token
///
/// This function generates a new token
pub fn new_token(
    key_pem: &[u8],
    alg: Algorithm,
    claims: JWTClaims,
) -> Result<String, Box<dyn std::error::Error>> {
    let header = Header::new(alg);
    let token = encode(&header, &claims, &EncodingKey::from_rsa_pem(key_pem)?)?;
    Ok(token)
}

/// # Validate Token
///
/// This function validates a token
pub fn validate_token(
    public_pem: &[u8],
    algorithm: Algorithm,
    token: &str,
) -> Result<JWTClaims, Errors> {
    // Set the validation parameters
    let mut validation = jsonwebtoken::Validation::new(algorithm);

    // Set the required claims
    validation.set_required_spec_claims(&["sub", "exp", "iss", "aud"]);

    debug!("🔑 Token Validation: {:?}", validation);
    debug!("🔑 Token: {}", token);

    let key = match DecodingKey::from_rsa_pem(public_pem) {
        Ok(key) => key,
        Err(_) => {
            return Err(Errors::InvalidToken);
        }
    };

    // Decode the token
    let token_data = match jsonwebtoken::decode::<JWTClaims>(
        token,
        &key,
        &jsonwebtoken::Validation::new(algorithm),
    ) {
        Ok(data) => data,
        Err(e) => match *e.kind() {
            _ => {
                return Err(Errors::InvalidToken);
            }
        },
    };

    debug!("🔑 Token Data: {:?}", token_data);
    if token_data.claims.exp < chrono::Utc::now().timestamp() {
        debug!("❌ Token expired");
        return Err(Errors::ExpiredToken);
    }

    Ok(token_data.claims)
}