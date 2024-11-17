// Title: main
// Created by sorryu
// Date: 2024-11-11
// Description: Program entry.

/*
History(ex: 20xx-xx-xx | Modifications(what, how, why) | name)
2024-11-18 | Settings environmental variables, database URL and server URL | sorryu

*/

use std::env;
use std::fs;

fn main() {
    // Read the `RUN_ENV` variable or use "development" as default
    let run_env = env::var("RUN_ENV").unwrap_or_else(|_| "default".to_string());
    let config_path = format!("config/{}.toml", run_env);

    // Read the content of the TOML file
    let config_content = fs::read_to_string(config_path)
        .expect("Failed to read the configuration file");

    // Parse the content using the `toml` crate
    let config: toml::Value = toml::from_str(&config_content)
        .expect("Failed to parse the configuration file");

    // Get the database URL
    if let Some(database_url) = config.get("database").and_then(|db| db.get("url")).and_then(|url| url.as_str()) {
        println!("cargo:rustc-env=DATABASE_URL={}", database_url);
    } else {
        panic!("DATABASE_URL not found in the configuration file");
    }

    // Get the server URL
    if let Some(server_url) = config.get("server").and_then(|srv| srv.get("url")).and_then(|url| url.as_str()) {
        println!("cargo:rustc-env=SERVER_URL={}", server_url);
    } else {
        panic!("SERVER_URL not found in the configuration file");
    }
}