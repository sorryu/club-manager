// Title: user_controller
// Created by sorryu
// Date: 2024-11-11
// Description: club creation and user-role management API handler

/*
History(ex: 20xx-xx-xx | Modifications(what, how, why) | name)
2024-11-17 | Create user registration and query API handler functions | sorryu
2024-11-17 | Add phone number in structure CreateUserRequest | sorryu
2024-11-17 | Insert user information into database | sorryu
2024-11-18 | Create User structure and Add database lookup logic | sorryu
2024-11-22 | Connect the correct structure | sorryu
2024-11-25 | use try_into and return Error | sorryu
2024-11-26 | connect global pool from utils::db_pool | sorryu
2024-11-26 | Separate database search logic, to be used in other places such as resolver | sorryu
2024-11-29 | Change sqlx::Error to Error | sorryu
2024-11-29 | change insert_user into public function | sorryu
2024-11-29 | Move the insert_user function, get_all_users function, which was on the controller to the service location | sorryu
2024-11-30 | Delete global pool and get parameter pool for web::Data<PgPool> | sorryu

*/

use actix_web::{get, post, web, HttpResponse, Responder};
// web: Provides multiple types and features for HTTP requests and responsese
// HttpResponse: Generates HTTP Response
// Responder: Return HTTP Response from Handler Function

use sqlx::PgPool;
use log::error;

use crate::{
    models::user::{ UserData, UserRequest, UserResponse },
    services::auth_service::{ get_all_users, insert_user },
};

// User registration handler
#[post("/api/users")] // Attribute macro, POST HTTP request method path
async fn create_user(pool: web::Data<PgPool>, user_request: web::Json<UserRequest>) -> impl Responder { // set Responder to Return Value
    let user_data = match user_request.into_inner().try_into() {
        Ok(data) => data,
        Err(err) => {
            error!("Error: {:?}", err);
            return HttpResponse::BadRequest()
                .body(format!("Invalid input: {}", err));
        }
    }; // Extract the inner data from web::Json

    match insert_user(pool.get_ref(), &user_data).await {
        Ok(_) => {
            HttpResponse::Ok()
                .json(format!("User {} created!", 
                    user_data.username
                    .clone()
                    .unwrap_or_else(|| "unknown".to_string())))
        },
        Err(err) => {
            error!("Error: {:?}", err);
            HttpResponse::InternalServerError()
                .body(format!("Cannot Create User: {}", err))
        },
    }
}

// All User lookup handler
#[get("/api/users")] // GET HTTP request method path
async fn get_users(pool: web::Data<PgPool>) -> impl Responder {
    match get_all_users(pool.get_ref()).await {
        Ok(users_data) => {
            let users_responses: Vec<UserResponse> = 
                users_data
                .into_iter()
                .map(UserData::into)
                .collect();
            HttpResponse::Ok().json(users_responses)
        },
        Err(err) => {
            error!("Error: {:?}", err);
            HttpResponse::InternalServerError().body(format!("Cannot get users from database: {}", err))
        }
    }
}

// Set the in-module handler
pub fn init_routes(cfg: &mut web::ServiceConfig) { // Configure services and routes for web applications
    cfg.service(create_user);
    cfg.service(get_users);
}