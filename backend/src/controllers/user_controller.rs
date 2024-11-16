// Title: user_controller
// Created by sorryu
// Date: 2024-11-11
// Description: club creation and user-role management API handler

/*
History(ex: 20xx-xx-xx | Modifications(what, how, why) | name)
2024-11-17 | Create user registration and query API handler functions | sorryu
2024-11-17 | Add phone number in structure CreateUserRequest | sorryu

*/

use actix_web::{get, post, web, HttpResponse, Responder};
// web: Provides multiple types and features for HTTP requests and responsese
// HttpResponse: Generates HTTP Response
// Responder: Return HTTP Response from Handler Function

use serde::{ Deserialize, Serialize };
// serde : Data serialization, deserialization

// data structures
#[derive(Deserialize)] // Convert data to rust structure
struct CreateUserRequest {
    username: String,
    email: String,
    password: String,
    number: String,
}

// User registration handler
#[post("/api/users")] // Attribute macro, POST HTTP request method path
async fn create_user(user_data: web::Json<CreateUserRequest>) -> impl Responder { // set Responder to Return Value
    // Need to add request data processing and database storage logic

    HttpResponse::Ok().json(format!("User {} created!", user_data.username)) // .json: Serialize HTTP responses in JSON format
}

// All User lookup handler
#[get("/api/users")] // GET HTTP request method path
async fn get_users() -> impl Responder {
    // need to add database lookup logic

    HttpResponse::Ok().json(vec!["user1", "user2", "user3"]) // vec!: Generate vector, Dynamic array of variable lengths
}

// Set the in-module handler
pub fn init_routes(cfg: &mut web::ServiceConfig) { // Configure services and routes for web applications
    cfg.service(create_user);
    cfg.service(get_users);
}