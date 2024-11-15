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

*/

use std::env;
use log::{info, warn, error};
use env_logger;

use config::Config;

use sqlx::PgPool;

mod controllers;
mod graphql;
mod models;
mod services;
mod utils;
mod websockets;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize the logger
    // preset log level is "info"
    // to set log level: RUST_LOG=info cargo run
    env_logger::init();

    info!("Application Starting...");

    // read environment variable RUN_ENV, preset is "default"
    // to set environment var: RUN_ENV=development cargo run
    let env = env::var("RUN_ENV").
        unwrap_or_else(|_| "default".to_string());

    // load environment settings file
    let settings = Config::builder().
        add_source(config::File::with_name(&format!("config/{}", env))).
        build()?;

    // get database URL
    let database_url: String = settings.
        get("database.url").
        expect("Database URL must be set in config file");

    // Generate connection pool for PostgreSQL
    let pool = PgPool::connect(&database_url).await?;

    info!("Loaded environment settings from config/{}.toml", env);
    info!("Connected to the database!");

    // other logics

    Ok(())
}