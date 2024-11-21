// Title: club
// Created by sorryu
// Date: 2024-11-11
// Description: Define a club model(fields: id, name, description, created_at, etc.)

/*
History(ex: 20xx-xx-xx | Modifications(what, how, why) | name)
2024-11-20 | Create ClubResponse, ClubRequest, ClubData | sorryu
2024-11-20 | Define transformation between structures | sorryu

*/

use serde::{Serialize, Deserialize};
use std::fmt::Debug;
use crate::models::user::{UserData, UserResponse};
use sqlx::{ Pool, Postgres, FromRow };

#[derive(Debug, Serialize, FromRow)]
pub struct ClubResponse {
    pub id: i32,
    pub name: String,
    pub creation_user: UserResponse,
    pub description: String,
}

#[derive(Debug, Deserialize, FromRow)]
pub struct ClubRequest {
    pub name: Option<String>,
    pub creation_userid: Option<i32>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ClubData {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub creation_userid: Option<i32>,
    pub description: Option<String>,
}

// `ClubRequest` -> `ClubData`
impl From<ClubRequest> for ClubData {
    fn from(req: ClubRequest) -> Self {
        ClubData {
            id: None,
            name: req.name,
            creation_userid: req.creation_userid,
            description: req.description,
        }
    }
}

// 비동기 변환 함수: `ClubData` -> `ClubResponse`
impl ClubData {
    pub async fn to_response(self, pool: &Pool<Postgres>) -> Result<ClubResponse, sqlx::Error> {
        let user = sqlx::query_as::<_, UserData>(
            "SELECT id, username, email, number, hashed_password FROM users WHERE id = $1",
        )
        .bind(self.creation_userid)
        .fetch_one(pool)
        .await?;

        Ok(ClubResponse {
            id: self.id.unwrap_or_default(),
            name: self.name.unwrap_or_default(),
            creation_user: user.into(),
            description: self.description.unwrap_or_default(),
        })
    }
}