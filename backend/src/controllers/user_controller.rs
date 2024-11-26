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

*/

use actix_web::{get, post, web, HttpResponse, Responder};
// web: Provides multiple types and features for HTTP requests and responsese
// HttpResponse: Generates HTTP Response
// Responder: Return HTTP Response from Handler Function

use sqlx::PgPool;
use log::error;

use crate::{
    models::user::{ UserData, UserRequest, UserResponse },
    utils::db_pool::pool,
};

// Insert user into database
async fn insert_user(db_pool: &PgPool, user_data: &UserData) -> Result<(), sqlx::Error> {
    sqlx::query!("INSERT INTO users (username, email, number, password_hash) VALUES ($1, $2, $3, $4)",
        user_data.username,
        user_data.email,
        user_data.number,
        user_data.hashed_password)
    .execute(db_pool)
    .await?;
    Ok(())
}

// User registration handler
#[post("/api/users")] // Attribute macro, POST HTTP request method path
async fn create_user(user_request: web::Json<UserRequest>) -> impl Responder { // set Responder to Return Value
    // Get request data
    let db_pool_guard = pool.lock().await;
    if let Some(db_pool) = &*db_pool_guard {
        let user_data = match user_request.into_inner().try_into() {
            Ok(data) => data,
            Err(err) => {
                return HttpResponse::BadRequest().body(format!("Invalid input: {}", err));
            }
        }; // Extract the inner data from web::Json

        match insert_user(db_pool, &user_data).await {
            Ok(_) => {
                HttpResponse::Ok()
                    .json(format!("User {} created!", user_data.username.clone().unwrap_or_else(|| "unknown".to_string())))
            },
            Err(err) => {
                error!("Error: {:?}", err);
                HttpResponse::InternalServerError().body(format!("Cannot Create User: {}", err))
            },
        }
    } else {
        HttpResponse::InternalServerError().body("Database pool not initialized!")
    }
}

pub async fn get_all_users(db_pool: &PgPool) -> Result<Vec<UserData>, sqlx::Error> {
    sqlx::query_as::<_, UserData>("SELECT id, username, number, hashed_password FROM users")
        .fetch_all(db_pool)
        .await
}

// All User lookup handler
#[get("/api/users")] // GET HTTP request method path
async fn get_users() -> impl Responder {
    let db_pool_guard = pool.lock().await;
    if let Some(db_pool) = &*db_pool_guard {
        match get_all_users(db_pool).await {
            Ok(users_data) => {
                let users_responses: Vec<UserResponse> = users_data.into_iter().map(UserData::into).collect();
                HttpResponse::Ok().json(users_responses)
            },
            Err(err) => {
                error!("Error: {:?}", err);
                HttpResponse::InternalServerError().body(format!("Cannot get users from database: {}", err))
            }
        }
    } else {
        HttpResponse::InternalServerError().body("Database pool is not initialized.")
    }
}

// Set the in-module handler
pub fn init_routes(cfg: &mut web::ServiceConfig) { // Configure services and routes for web applications
    cfg.service(create_user);
    cfg.service(get_users);
}