// Title: db_pool
// Created by sorryu
// Date: 2024-11-20
// Description: Define database pool

/*
History(ex: 20xx-xx-xx | Modifications(what, how, why) | name)
2024-11-20 | Create pool from database | sorryu
2024-11-20 | Treat pool as global variable | sorryu
2024-11-29 | Change pool to upper case name, 'Pool' | sorryu
2024-11-30 | Delete global Pool and Modify initialize_pool to return database_pool | sorryu

*/

use sqlx::{ PgPool, Error as SqlError, Pool, Postgres };
use log::{ error, info };

use crate::utils::settings::get_database_url;

pub type DbPool = Pool<Postgres>;

pub async fn initialize_pool() -> Result<PgPool, SqlError> {
    let database_url = get_database_url()
        .await
        .map_err(|err| SqlError::Protocol(format!("Database pool is not initialized: {}", err)))?;

    match PgPool::connect(&database_url).await {
        Ok(database_pool) => {
            info!("Database pool initialized successfully.");
            Ok(database_pool)
        }
        Err(e) => {
            error!("Failed to create database pool: {:?}", e);
            Err(e)
        }
    }
}