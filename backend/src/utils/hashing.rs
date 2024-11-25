// Title: hashing
// Created by sorryu
// Date: 2024-11-11
// Description: Password hashing and verification utility

/*
History(ex: 20xx-xx-xx | Modifications(what, how, why) | name)
2024-11-19 | Add fake hash function | sorryu
2024-11-25 | Create hash_password and is_valid_password | sorryu

*/

use bcrypt::{ hash, verify, DEFAULT_COST };
use log::{ info, error };

pub fn hash_password(password: &str) -> Result<String, String> {
    match hash(password, DEFAULT_COST) {
        Ok(hashed) => Ok(hashed),
        Err(e) => {
            error!("Failed to hash password: {}", e);
            Err("Failed to hash password".to_string())
        }
    }
}

pub fn is_valid_password(password: &str, hashed_password: &str) -> Result<bool, String> {
    match verify(password, hashed_password) {
        Ok(is_valid) => {
            if is_valid {
                info!("Password is valid");
            } else {
                error!("Password is invalid");
            }
            Ok(is_valid)
        },
        Err(e) => {
            error!("Failed to verify password: {:?}", e);
            Err("Failed to verify password".to_string())
        }
    }
}