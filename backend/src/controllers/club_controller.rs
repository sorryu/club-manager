// Title: club_controller
// Created by sorryu
// Date: 2024-11-11
// Description: User registration and login API handler

/*
History(ex: 20xx-xx-xx | Modifications(what, how, why) | name)
2024-11-17 | Create club Creation and query API handler functions | sorryu
2024-11-18 | Create User structure, and Add insert logic of user information into database database lookup logic | sorryu

*/

use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{ Deserialize, Serialize };
use sqlx::PgPool;

// data structures
// club data request
#[derive(Deserialize)]
struct CreateClubRequest {
    creation_userid: i32,
    name: String,
    description: String,
}

// Club structures queried in the database
#[derive(Serialize, sqlx::FromRow)]
struct Club {
    id: i32,
    name: String,
    creation_userid: i32,
    description: String,
}

// Insert club into database
async fn insert_club(pool: &PgPool, club_data: &CreateClubRequest) -> Result<(), sqlx::Error> {
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
async fn create_club(pool: web::Data<PgPool>, club_data: web::Json<CreateClubRequest>) -> impl Responder {
    // need to add request data processing and database storage logic
    // Get request data
    let club = club_data.into_inner();

    match insert_club(pool.get_ref(), &club).await {
        Ok(_) => {
            HttpResponse::Ok()
                .json(format!("Club {} created!", club.name))
        },
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Error: {}", err))
        },
    };

    HttpResponse::Ok().json(format!("Club {} created!", club.name))
}

// All Club lookup handler
#[get("/api/clubs")]
async fn get_clubs(pool: web::Data<PgPool>) -> impl Responder {
    // need to add database lookup logic
    let result = sqlx::query_as::<_, Club>("SELECT id, name, creation_userid, description FROM clubs")
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(clubs) => HttpResponse::Ok().json(clubs),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err))
    }
}

// Set the in-module handler
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_club);
    cfg.service(get_clubs);
}