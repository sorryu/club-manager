// Title: user
// Created by sorryu
// Date: 2024-11-11
// Description: Define a user model(fields: id, username, email, password_hash, created_at, etc.)

/*
History(ex: 20xx-xx-xx | Modifications(what, how, why) | name)
2024-11-18 | Create default user structure, insert structure, response and request structure | sorryu
2024-11-19 | Integration by UserResponse, UserRequest | sorryu
2024-11-20 | Create UserData and define transformation between structures | sorryu

*/

use serde::{ Serialize, Deserialize };
use std::fmt::Debug;

use crate::utils::hashing::hash_password;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub number: String,
}

#[derive(Debug, Deserialize, sqlx::FromRow)]
pub struct UserRequest {
    pub username: Option<String>,
    pub email: String,
    pub number: Option<String>,
    pub password: String,
}

// Convert from/to Database
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserData {
    pub id: Option<i32>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub number: Option<String>,
    pub hashed_password: Option<String>
}

// `UserRequest` -> `UserData`
impl From<UserRequest> for UserData {
    fn from(req: UserRequest) -> Self {
        let hashed_password = hash_password(&req.password);
        UserData {
            id: None,
            username: req.username,
            email: Some(req.email),
            number: req.number,
            hashed_password: Some(hashed_password),
        }
    }
}

// `UserData` -> `UserResponse`
impl From<UserData> for UserResponse {
    fn from(data: UserData) -> Self {
        UserResponse {
            id: data.id.unwrap_or_default(),
            username: data.username.unwrap_or_default(),
            email: data.email.unwrap_or_default(),
            number: data.number.unwrap_or_default(),
        }
    }
}