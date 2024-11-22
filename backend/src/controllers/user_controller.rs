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

*/

use actix_web::{get, post, web, HttpResponse, Responder};
// web: Provides multiple types and features for HTTP requests and responsese
// HttpResponse: Generates HTTP Response
// Responder: Return HTTP Response from Handler Function

use sqlx::PgPool;

use crate::models::user::{ UserData, UserRequest, UserResponse };

// Insert user into database
async fn insert_user(pool: &PgPool, user_data: &UserData) -> Result<(), sqlx::Error> {

    sqlx::query!("INSERT INTO users (username, email, number, password_hash) VALUES ($1, $2, $3, $4)",
        user_data.username,
        user_data.email,
        user_data.number,
        user_data.hashed_password)
    .execute(pool)
    .await?;
    Ok(())
}

// User registration handler
#[post("/api/users")] // Attribute macro, POST HTTP request method path
async fn create_user(pool: web::Data<PgPool>, user_request: web::Json<UserRequest>) -> impl Responder { // set Responder to Return Value
    // Get request data
    let user_data = user_request.into_inner().into(); // Extract the inner data from web::Json

    match insert_user(pool.get_ref(), &user_data).await {
        Ok(_) => {
            HttpResponse::Ok()
                .json(format!("User {} created!", user_data.username.clone().unwrap_or_else(|| "unknown".to_string())))
        },
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Error: {}", err))
        },
    }
}

// All User lookup handler
#[get("/api/users")] // GET HTTP request method path
async fn get_users(pool: web::Data<PgPool>) -> impl Responder {
    // need to add database lookup logic
    let result = sqlx::query_as::<_, UserData>("SELECT id, username, number, hashed_password FROM users")
        .fetch_all(pool.get_ref())
        .await;
    
    match result {
        Ok(users_data) => {
            let users_response: Vec<UserResponse> = users_data.into_iter().map(UserData::into).collect();
            HttpResponse::Ok().json(users_response)
        },
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Error: {}", err))
        },
    }
}

// Set the in-module handler
pub fn init_routes(cfg: &mut web::ServiceConfig) { // Configure services and routes for web applications
    cfg.service(create_user);
    cfg.service(get_users);
}