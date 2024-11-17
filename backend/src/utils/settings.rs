// Title: settings
// Created by sorryu
// Date: 2024-11-18
// Description: database utils to connect pool

/*
History(ex: 20xx-xx-xx | Modifications(what, how, why) | name)
2024-11-18 | References to environment variables and return database URL | sorryu
2024-11-18 | return server URL | sorryu

*/

use std::env;
use config::Config;

use log::{trace, info, error};
// use log::{debug, warn};

fn load_settings() -> Config {
    // read environment variable RUN_ENV, preset is "default"
    // to set environment var: RUN_ENV=development cargo run
    let env = env::var("RUN_ENV")
        .unwrap_or_else(|_| "default".to_string());

    trace!("Loading environment settings from config/{}.toml", env);

    // Return environment settings file
    Config::builder()
        .add_source(config::File::with_name(&format!("config/{}", env)))
        .build()
        .expect("Failed to load configuration file")
}

pub async fn get_database_url() -> Result<String, Box<dyn std::error::Error>> {
    // load environment settings file
    let settings = load_settings();

    // Return Database url
    match settings.get::<String>("database.url") {
        Ok(url) => {
            info!("Database URL successfully retrieved.");
            Ok(url)
        },
        Err(e) => {
            error!("Failed to retrieve database URL: {:?}", e);
            Err(Box::new(e))
        }
    }
}

pub async fn get_server_url() -> Result<String, Box<dyn std::error::Error>> {
    // load environment settings file
    let settings = load_settings();

    // Return web server url
    match settings.get::<String>("server.url") {
        Ok(url) => {
            info!("Server URL successfully retrieved.");
            Ok(url)
        },
        Err(e) => {
            error!("Failed to retrieve server URL: {:?}", e);
            Err(Box::new(e))
        }
    }
}