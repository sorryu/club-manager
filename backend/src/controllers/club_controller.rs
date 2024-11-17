// Title: club_controller
// Created by sorryu
// Date: 2024-11-11
// Description: User registration and login API handler

/*
History(ex: 20xx-xx-xx | Modifications(what, how, why) | name)
2024-11-17 | Create club Creation and query API handler functions | sorryu

*/

use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{ Deserialize, Serialize };

// data structures
#[derive(Deserialize)]
struct CreateClubRequest {
    creation_userid: i128,
    name: String,
    description: String,
}

// Club creation handler
#[post("/api/clubs")]
async fn create_club(club_data: web::Json<CreateClubRequest>) -> impl Responder {
    // need to add request data processing and database storage logic

    HttpResponse::Ok().json(format!("Club {} created!", club_data.name))
}

// All Club lookup handler
#[get("/api/clubs")]
async fn get_clubs() -> impl Responder {
    // need to add database lookup logic

    HttpResponse::Ok().json(vec!["club1", "club2", "club3"])
}

// Set the in-module handler
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_club);
    cfg.service(get_clubs);
}