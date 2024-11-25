// Title: db_pool
// Created by sorryu
// Date: 2024-11-20
// Description: Define database pool

/*
History(ex: 20xx-xx-xx | Modifications(what, how, why) | name)
2024-11-20 | Create pool from database | sorryu
2024-11-20 | Treat pool as global variable | sorryu

*/

use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;
use once_cell::sync::Lazy;
use crate::utils::settings::get_database_url;
use log::{error, info};

pub type GlobalPgPool = Arc<Mutex<Option<PgPool>>>;

// Global PgPool
pub static pool: Lazy<GlobalPgPool> = Lazy::new(|| {Arc::new(Mutex::new(None))});

pub async fn initialize_pool() {
    let database_url = match get_database_url().await {
        Ok(url) => url,
        Err(e) => {
            error!("Failed to get database URL: {:?}", e);
            return;
        }
    };

    match PgPool::connect(&database_url).await {
        Ok(database_pool) => {
            let mut db_pool_guard = pool.lock().await;
            *db_pool_guard = Some(database_pool);
            info!("Database pool initialized successfully.");
        }
        Err(e) => {
            error!("Failed to create database pool: {:?}", e);
        }
    }
}