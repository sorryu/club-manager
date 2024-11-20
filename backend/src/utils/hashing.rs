// Title: hashing
// Created by sorryu
// Date: 2024-11-11
// Description: Password hashing and verification utility

/*
History(ex: 20xx-xx-xx | Modifications(what, how, why) | name)
2024-11-19 | Add fake hash function | sorryu

*/

pub fn hash_password(password: &str) -> String {
    format!("hashed_{}", password) // 실제로는 bcrypt 등의 라이브러리 사용 필요
}