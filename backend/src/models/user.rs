// Title: user
// Created by sorryu
// Date: 2024-11-11
// Description: Define a user model(fields: id, username, email, password_hash, created_at, etc.)

/*
History(ex: 20xx-xx-xx | Modifications(what, how, why) | name)
2024-11-18 | Create default user structure, insert structure, response and request structure | sorryu
2024-11-19 | Integration by UserResponse, UserRequest | sorryu
2024-11-20 | Create UserData and define transformation between structures | sorryu
2024-11-25 | Convert From to TryFrom for UserRequest to UserData | sorryu
2024-11-29 | Add parameters FromRow, SimpleObject, and InputObject to the derive to handle the input/output of the database and graphpl | sorryu
2024-12-02 | Convert Users, number to phone_number | sorryu

*/

use serde::{ Serialize, Deserialize };
use std::{ fmt::Debug, convert::TryFrom };
use sqlx::FromRow;
use async_graphql::{ SimpleObject, InputObject };

use crate::utils::hashing::hash_password;

#[derive(Debug, Serialize, FromRow, SimpleObject, InputObject)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub phone_number: String,
}

#[derive(Debug, Deserialize, FromRow, SimpleObject, InputObject)]
pub struct UserRequest {
    pub username: Option<String>,
    pub email: String,
    pub phone_number: Option<String>,
    pub password: String,
}

// Convert from/to Database
#[derive(Debug, Serialize, Deserialize, FromRow, SimpleObject, InputObject)]
pub struct UserData {
    pub id: Option<i32>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub hashed_password: Option<String>
}

// `UserRequest` -> `UserData`
impl TryFrom<UserRequest> for UserData {
    type Error = String;

    fn try_from(req: UserRequest) -> Result<Self, Self::Error> {
        let hashed_password = hash_password(&req.password)
            .map_err(|e| format!("Password hashing failed: {}", e))?;

        Ok(UserData {
            id: None,
            username: req.username,
            email: Some(req.email),
            phone_number: req.phone_number,
            hashed_password: Some(hashed_password),
        })
    }
}

// `UserData` -> `UserResponse`
impl From<UserData> for UserResponse {
    fn from(data: UserData) -> Self {
        UserResponse {
            id: data.id.unwrap_or_default(),
            username: data.username.unwrap_or_default(),
            email: data.email.unwrap_or_default(),
            phone_number: data.phone_number.unwrap_or_default(),
        }
    }
}