// Title: club
// Created by sorryu
// Date: 2024-11-11
// Description: Define a club model(fields: id, name, description, created_at, etc.)

/*
History(ex: 20xx-xx-xx | Modifications(what, how, why) | name)
2024-11-20 | Create ClubResponse, ClubRequest, ClubData | sorryu
2024-11-20 | Define transformation between structures | sorryu
2024-11-29 | Add parameters FromRow, SimpleObject, and InputObject to the derive to handle the input/output of the database and graphpl | sorryu
2024-12-02 | Exclude Option from ClubRequest Structure | sorryu
2024-12-05 | Change the type of creation_userid and description to optional among the elements in ClubRequest | sorryu

*/

use serde::{Serialize, Deserialize};
use async_graphql::{ SimpleObject, InputObject };
use std::fmt::Debug;
use sqlx::{ PgPool, Pool, Postgres, FromRow, Error as SqlError };
use log::error;

use crate::models::user::{UserData, UserResponse};

#[derive(Debug, Serialize, FromRow, SimpleObject, InputObject)]
pub struct ClubResponse {
    pub id: i32,
    pub name: String,
    pub creation_user: UserResponse,
    pub description: String,
}

#[derive(Debug, Deserialize, FromRow, SimpleObject, InputObject)]
pub struct ClubRequest {
    pub name: String,
    pub creation_userid: Option<i32>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, SimpleObject, InputObject)]
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
            name: Some(req.name),
            creation_userid: req.creation_userid,
            description: req.description,
        }
    }
}

// 비동기 변환 함수: `ClubData` -> `ClubResponse`
impl ClubData {
    pub async fn to_response(self, pool: &Pool<Postgres>) -> Result<ClubResponse, SqlError> {
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

pub async fn convert_to_response(club_data: ClubData, pool: &PgPool) -> Option<ClubResponse> {
    match club_data.to_response(pool).await {
        Ok(club_response) => Some(club_response),
        Err(err) => {
            error!("Failed to convert club data to response: {:?}", err);
            None
        }
    }
}