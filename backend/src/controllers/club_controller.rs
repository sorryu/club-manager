// Title: club_controller
// Created by sorryu
// Date: 2024-11-11
// Description: User registration and login API handler

/*
History(ex: 20xx-xx-xx | Modifications(what, how, why) | name)
2024-11-17 | Create club Creation and query API handler functions | sorryu
2024-11-18 | Create User structure, and Add insert logic of user information into database database lookup logic | sorryu
2024-11-22 | Connect the correct structure from crate:: | sorryu
2024-11-26 | connect global pool from utils::db_pool | sorryu
2024-11-26 | Separate database search logic, to be used in other places such as resolver | sorryu

*/

use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use log::error;

use crate::{
    models::club::{ ClubData, ClubRequest, ClubResponse },
    utils::db_pool::pool,
};

// Insert club into database
async fn insert_club(db_pool: &PgPool, club_data: &ClubData) -> Result<(), sqlx::Error> {
    sqlx::query!("INSERT INTO clubs (name, creation_userid, description) VALUES ($1, $2, $3)",
        club_data.name,
        club_data.creation_userid,
        club_data.description)
    .execute(db_pool)
    .await?;
    Ok(())
}

// Club creation handler
#[post("/api/clubs")]
async fn create_club(club_request: web::Json<ClubRequest>) -> impl Responder {
    let db_pool_guard = pool.lock().await;
    if let Some(db_pool) = &*db_pool_guard {
        // need to add request data processing and database storage logic
        // Get request data
        let club_data = club_request.into_inner().into();

        match insert_club(db_pool, &club_data).await {
            Ok(_) => {
                HttpResponse::Ok()
                    .json(format!("Club {} created!", club_data.name.clone().unwrap_or_else(|| "unknown".to_string())))
            },
            Err(err) => {
                error!("Error: {:?}", err);
                HttpResponse::InternalServerError().body(format!("Cannot Create Club: {}", err))
            },
        }
    } else {
        HttpResponse::InternalServerError().body("Database pool not initialized!")
    }
}

pub async fn get_all_clubs(db_pool: &PgPool) -> Result<Vec<ClubData>, sqlx::Error> {
    sqlx::query_as::<_, ClubData>("SELECT id, name, creation_userid, description FROM clubs")
        .fetch_all(db_pool)
        .await
}

// All Club lookup handler
#[get("/api/clubs")]
async fn get_clubs() -> impl Responder {
    let db_pool_guard = pool.lock().await;
    if let Some(db_pool) = &*db_pool_guard {
        match get_all_clubs(db_pool).await {
            Ok(clubs_data) => {
                let clubs_response: Vec<ClubResponse> = {
                    let mut responses = Vec::new();
                    for club_data in clubs_data {
                        match club_data.to_response(db_pool).await {
                            Ok(response) => responses.push(response),
                            Err(err) => {
                                log::error!("Failed to convert club data to response: {:?}", err);
                                return HttpResponse::InternalServerError().body("Failed to fetch clubs.");
                            }
                        }
                    }
                    responses
                };
                HttpResponse::Ok().json(clubs_response)
            },
            Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err))
        }
    } else {
        HttpResponse::InternalServerError().body("Database pool is not initialized.")
    }
    
}

// Set the in-module handler
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_club);
    cfg.service(get_clubs);
}