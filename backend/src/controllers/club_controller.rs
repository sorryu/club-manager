// Title: club_controller
// Created by sorryu
// Date: 2024-11-11
// Description: User registration and login API handler

/*
History(ex: 20xx-xx-xx | Modifications(what, how, why) | name)
2024-11-17 | Create club Creation and query API handler functions | sorryu
2024-11-18 | Create User structure, and Add insert logic of user information into database database lookup logic | sorryu
2024-11-22 | Connect the correct structure from crate:: | sorryu

*/

use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::models::club::{ ClubData, ClubRequest, ClubResponse };

// Insert club into database
async fn insert_club(pool: &PgPool, club_data: &ClubData) -> Result<(), sqlx::Error> {
    sqlx::query!("INSERT INTO clubs (name, creation_userid, description) VALUES ($1, $2, $3)",
        club_data.name,
        club_data.creation_userid,
        club_data.description)
    .execute(pool)
    .await?;
    Ok(())
}

// Club creation handler
#[post("/api/clubs")]
async fn create_club(pool: web::Data<PgPool>, club_request: web::Json<ClubRequest>) -> impl Responder {
    // need to add request data processing and database storage logic
    // Get request data
    let club_data = club_request.into_inner().into();

    match insert_club(pool.get_ref(), &club_data).await {
        Ok(_) => {
            HttpResponse::Ok()
                .json(format!("Club {} created!", club_data.name.clone().unwrap_or_else(|| "unknown".to_string())))
        },
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Error: {}", err))
        },
    }
}

// All Club lookup handler
#[get("/api/clubs")]
async fn get_clubs(pool: web::Data<PgPool>) -> impl Responder {
    // need to add database lookup logic
    let result = sqlx::query_as::<_, ClubData>("SELECT id, name, creation_userid, description FROM clubs")
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(clubs_data) => {
            let clubs_response: Vec<ClubResponse> = {
                let mut responses = Vec::new();
                for club_data in clubs_data {
                    match club_data.to_response(pool.get_ref()).await {
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
}

// Set the in-module handler
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_club);
    cfg.service(get_clubs);
}