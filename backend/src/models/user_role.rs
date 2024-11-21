// Title: user_role.rs
// Created by sorryu
// Date: 2024-11-11
// Description: Define a user-club relationship model(fields: id, userid, clubid, role, etc.)

/*
History(ex: 20xx-xx-xx | Modifications(what, how, why) | name)
2024-11-21 | Create ClubResponse, ClubRequest, and ClubData, and Define transformation between structures | sorryu

*/

use serde::{ Serialize, Deserialize };
use std::fmt::Debug;
use crate::models::{
    user::{ UserData, UserResponse }, 
    club::{ ClubData, ClubResponse }
};
use sqlx::{ Pool, Postgres, FromRow };

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct UserRoleResponse {
    pub id: i32,
    pub user: UserResponse,
    pub club: ClubResponse,
    pub role: String,
}

#[derive(Debug, Deserialize, FromRow)]
pub struct UserRoleRequest {
    pub user_id: Option<i32>,
    pub club_id: Option<i32>,
    pub role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserRoleData {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub club_id: Option<i32>,
    pub role: Option<String>,
}

// `UserRoleRequest` -> `UserRoleData`
impl From<UserRoleRequest> for UserRoleData {
    fn from(req: UserRoleRequest) -> Self {
        UserRoleData {
            id: None,
            user_id: req.user_id,
            club_id: req.club_id,
            role: req.role,
        }
    }
}

// `UserRoleData` -> `UserRoleResponse`
impl UserRoleData {
    pub async fn to_response(self, pool: &Pool<Postgres>) -> Result<UserRoleResponse, sqlx::Error> {
        let user = sqlx::query_as::<_, UserData>(
            "SELECT id, username, email, number, hashed_password FROM users WHERE id = $1"
        )
        .bind(self.user_id)
        .fetch_one(pool)
        .await?;

        let club = sqlx::query_as::<_, ClubData>(
            "SELECT id, name, creation_userid, description FROM clubs WHERE id = $1"
        )
        .bind(self.club_id)
        .fetch_one(pool)
        .await?;

        Ok(UserRoleResponse {
            id: self.id.unwrap_or_default(),
            user: user.into(),
            club: club.to_response(pool).await?,
            role: self.role.unwrap_or_default(),
        })
    }
}