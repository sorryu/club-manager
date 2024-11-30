// Title: auth_service
// Created by sorryu
// Date: 2024-11-11
// Description: User registration and login logic (password hashing, authentication token generation)

/*
History(ex: 20xx-xx-xx | Modifications(what, how, why) | name)
2024-11-29 | Move the insert_user function, get_all_users function, which was on the controller to the service location | sorryu

*/

use sqlx::{ PgPool, Error };
use crate::models::user::UserData;

// Insert user into database
pub async fn insert_user(db_pool: &PgPool, user_data: &UserData) -> Result<(), Error> {
    sqlx::query!("INSERT INTO users (username, email, number, password_hash) VALUES ($1, $2, $3, $4)",
        user_data.username,
        user_data.email,
        user_data.number,
        user_data.hashed_password)
    .execute(db_pool)
    .await?;
    Ok(())
}

pub async fn get_all_users(db_pool: &PgPool) -> Result<Vec<UserData>, Error> {
    sqlx::query_as::<_, UserData>("SELECT id, username, number, hashed_password FROM users")
        .fetch_all(db_pool)
        .await
}