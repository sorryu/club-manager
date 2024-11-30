// Title: club_service
// Created by sorryu
// Date: 2024-11-11
// Description: club creation and user-role management logic

/*
History(ex: 20xx-xx-xx | Modifications(what, how, why) | name)
2024-11-29 | Move the insert_club function, get_all_clubs function, which was on the controller to the service location | sorryu

*/

use crate::models::club::ClubData;

use sqlx::{ PgPool, error::Error as SqlError };

// Insert club into database
pub async fn insert_club(db_pool: &PgPool, club_data: &ClubData) -> Result<(), SqlError> {
    // Hard coding makes maintenance difficult. Let's consider ORM.
    sqlx::query!("INSERT INTO clubs (name, creation_userid, description) VALUES ($1, $2, $3)",
        club_data.name,
        club_data.creation_userid,
        club_data.description)
    .execute(db_pool)
    .await?;
    Ok(())
}

pub async fn get_all_clubs(db_pool: &PgPool) -> Result<Vec<ClubData>, SqlError> {
    // Let's receive additional parameters to enable filtering and paging.
    sqlx::query_as::<_, ClubData>("SELECT id, name, creation_userid, description FROM clubs")
        .fetch_all(db_pool)
        .await
}