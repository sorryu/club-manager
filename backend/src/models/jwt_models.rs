// Title: jwt_models
// Created by sorryu
// Date: 2024-11-22
// Description: define jwt models

/*
History(ex: 20xx-xx-xx | Modifications(what, how, why) | name)
2024-11-22 | Create structure Claims | sorryu

*/

use serde::{ Serialize, Deserialize };
use std::fmt::Debug;
pub use jsonwebtoken::Header;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub access: i8, // access rate, 0: free plan user, 1: plus plan user, 2: manager, 3: developer
    pub id: i32,
    pub email: String,
    pub exp: usize, // token expiration time
    pub iat: usize, // token issuance time
}