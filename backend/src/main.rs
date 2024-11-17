// Title: main
// Created by sorryu
// Date: 2024-11-11
// Description: Program entry.

/*
History(ex: 20xx-xx-xx | Modifications(what, how, why) | name)
2024-11-11 | Module connection and main function creation | sorryu
2024-11-12 | Add and Load environment settings, from default.toml | sorryu
2024-11-15 | Split environment settings into development and production environment | sorryu
2024-11-15 | Add log | sorryu
2024-11-17 | Add starting web server and routes of club and user controllers | sorryu
2024-11-18 | Delete env and change to get url with settings | sorryu

*/

use log::{trace, info, error};
// use log::{debug, warn};
use env_logger;

use actix_web::{App, HttpServer, web};
use sqlx::PgPool;

mod controllers;
mod graphql;
mod models;
mod services;
mod utils;
mod websockets;

use utils::settings::{get_database_url, get_server_url};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize the logger
    // preset log level is "info"
    // to set log level: RUST_LOG=info cargo run
    env_logger::init();

    trace!("Application Starting...");

    // Get the database URL
    let database_url = match get_database_url().await {
        Ok(url) => url,
        Err(e) => {
            error!("Failed to get database URL: {:?}", e);
            return Err(e);
        }
    };

    info!("Get database URL from environment settings!");

    // Generate connection pool for PostgreSQL
    let pool = PgPool::connect(&database_url).await?;

    info!("Connected to the database!");

    // Get the web server url
    let web_server_url = match get_server_url().await {
        Ok(url) => url,
        Err(e) => {
            error!("Failed to get web server URL: {:?}", e);
            return Err(e);
        }
    };

    // starting web server
    HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone())) // pass database pool to app
            .configure(controllers::user_controller::init_routes)
            .configure(controllers::club_controller::init_routes)
    }).bind(web_server_url)?.run().await?;

    // other logics

    Ok(())
}