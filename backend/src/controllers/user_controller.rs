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

*/

use actix_web::{get, post, web, HttpResponse, Responder};
// web: Provides multiple types and features for HTTP requests and responsese
// HttpResponse: Generates HTTP Response
// Responder: Return HTTP Response from Handler Function

use serde::{ Deserialize, Serialize };
// serde : Data serialization, deserialization

use sqlx::PgPool;

// data structures
// user request
#[derive(Deserialize)] // Convert data to rust structure
struct CreateUserRequest {
    username: String,
    email: String,
    hashed_password: String,
    number: String,
}

// User structures queried in the database
#[derive(Serialize, sqlx::FromRow)] // Convert rust data to others
struct User {
    id: i32,
    username: String,
    email: String,
    number: String,
}

// Insert user into database
async fn insert_user(pool: &PgPool, user_data: &CreateUserRequest) -> Result<(), sqlx::Error> {
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
async fn create_user(pool: web::Data<PgPool>, user_data: web::Json<CreateUserRequest>) -> impl Responder { // set Responder to Return Value
    // Need to add request data processing and database storage logic
    // Get request data
    let user = user_data.into_inner(); // Extract the inner data from web::Json

    match insert_user(pool.get_ref(), &user).await {
        Ok(_) => {
            HttpResponse::Ok()
                .json(format!("User {} created!", user.username))
        },
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Error: {}", err))
        },
    };

    HttpResponse::Ok().json(format!("User {} created!", user.username)) // .json: Serialize HTTP responses in JSON format
}

// All User lookup handler
#[get("/api/users")] // GET HTTP request method path
async fn get_users(pool: web::Data<PgPool>) -> impl Responder {
    // need to add database lookup logic
    let result = sqlx::query_as::<_, User>("SELECT id, username, email, number FROM users")
        .fetch_all(pool.get_ref())
        .await;
    
    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}

// Set the in-module handler
pub fn init_routes(cfg: &mut web::ServiceConfig) { // Configure services and routes for web applications
    cfg.service(create_user);
    cfg.service(get_users);
}