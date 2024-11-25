// Title: auth
// Created by sorryu
// Date: 2024-11-11
// Description: Utility functions related to JWT creation and authentication.

/*
History(ex: 20xx-xx-xx | Modifications(what, how, why) | name)
2024-11-22 | create functions generate_jwt, validate_jwt, extract_user_from_jwt, calculate_expiry, refresh_jwt | sorryu
2024-11-25 | create function generate_claims, modify generate_jwt to use claims instead of UserData | sorryu

*/

use jsonwebtoken as jwt;
use jwt::{ encode, decode, Algorithm, EncodingKey, DecodingKey, Validation, errors::{ Error, ErrorKind }};
use chrono::Utc;
use std::time::{ SystemTime, UNIX_EPOCH };
use crate::models::{
    jwt_models::{ Claims, Header },
    user::UserData
};

pub async fn generate_claims(user: UserData, exp_sec: u64) -> Result<Claims, Error> {
    let userid = user.id.ok_or_else(|| Error::from(ErrorKind::InvalidToken))?;
    let useremail = user.email.ok_or_else(|| Error::from(ErrorKind::InvalidToken))?;

    Ok(Claims {
        access: 0, // default is 0
        id: userid,
        email: useremail,
        exp: calculate_expiry(exp_sec as usize),
        iat: SystemTime::now().duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize
    })
}

pub async fn generate_jwt(claims: Claims, secret_key: &str) -> Result<String, Error> {
    let header = Header {
        alg: Algorithm::HS512,
        typ: Some("JWT".to_string()),
        ..Default::default()
    };

    encode(&header, &claims, &EncodingKey::from_secret(secret_key.as_ref()))
}

pub async fn validate_jwt(jwt_token: &str, secret_key: &str) -> Result<Claims, Error> {
    let mut validation = Validation::new(Algorithm::HS512);
    validation.validate_exp = true;

    let key = DecodingKey::from_secret(secret_key.as_bytes());

    let token_data = decode::<Claims>(jwt_token, &key, &validation)?;

    Ok(token_data.claims)
}

pub async fn extract_email(jwt_token: &str, secret_key: &str) -> Result<String, Error> {
    let claims = validate_jwt(jwt_token, secret_key).await?;

    Ok(claims.email) // ID(email)
}

pub async fn extract_access(jwt_token: &str, secret_key: &str) -> Result<i8, Error> {
    let claims = validate_jwt(jwt_token, secret_key).await?;

    Ok(claims.access)
}

pub fn calculate_expiry(exp_sec: usize) -> usize {
    (Utc::now().timestamp() as usize) + exp_sec
}

pub async fn refresh_jwt(jwt_token: &str, secret_key: &str, new_exp: u64) -> Result<String, Error> {
    let mut claims = validate_jwt(jwt_token, secret_key).await?;

    claims.exp = calculate_expiry(new_exp as usize);

    let new_jwt_token = generate_jwt(claims, secret_key).await?;

    Ok(new_jwt_token)
}