use actix_web::{error::ErrorUnauthorized, http::header::HeaderValue, Error};
use base64::{engine::general_purpose, Engine as _};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

pub fn decode_token(token: HeaderValue) -> Result<TokenClaims, Error> {
    match general_purpose::STANDARD_NO_PAD.decode(token.to_str().unwrap().replace("Bearer ", "")) {
        Ok(token_str) => {
            match decode::<TokenClaims>(
                String::from_utf8(token_str).unwrap().as_str(),
                &DecodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_ref()),
                &Validation::default(),
            ) {
                Ok(data) => Ok(data.claims),
                Err(_) => Err(ErrorUnauthorized("Invalid or missing authorization token.")),
            }
        }
        Err(_) => Err(ErrorUnauthorized("Invalid or missing authorization token.")),
    }
}
