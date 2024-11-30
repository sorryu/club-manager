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
2024-11-29 | Change sqlx::Error to Error | sorryu
2024-11-29 | change insert_club into public function | sorryu
2024-11-29 | Move the insert_club function, get_all_clubs function, which was on the controller to the service location | sorryu
2024-11-30 | Delete global pool and get parameter pool for web::Data<PgPool> | sorryu

*/

use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use log::error;

use crate::{
    models::club::{ ClubRequest, convert_to_response },
    services::club_service::{ insert_club, get_all_clubs },
};

// Club creation handler
#[post("/api/clubs")]
async fn create_club(pool: web::Data<PgPool>, club_request: web::Json<ClubRequest>) -> impl Responder {
    let club_data = match club_request.into_inner().try_into() {
        Ok(data) => data,
        Err(err) => {
            error!("Error: {:?}", err);
            return HttpResponse::BadRequest()
                .body(format!("Invalid input: {}", err));
        }
    };

    match insert_club(pool.get_ref(), &club_data).await {
        Ok(_) => {
            HttpResponse::Ok()
                .json(format!("Club {} created!", 
                    club_data.name
                    .clone()
                    .unwrap_or_else(|| "unknown".to_string())))
        },
        Err(err) => {
            error!("Error: {:?}", err);
            HttpResponse::InternalServerError()
                .body(format!("Cannot Create Club: {}", err))
        }
    }
}

// All Club lookup handler
#[get("/api/clubs")]
async fn get_clubs(pool: web::Data<PgPool>) -> impl Responder {
    match get_all_clubs(pool.get_ref()).await {
        Ok(clubs_data) => {
            let mut clubs_response = Vec::new();

            for club_data in clubs_data {
                clubs_response
                    .push(convert_to_response(club_data, pool.get_ref()).await);
            }

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