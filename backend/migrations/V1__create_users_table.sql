-- Title: create_users_table
-- Created by sorryu
-- Date: 2024-11-11
-- Description: Users table creation migration file.

/*
History(ex: 20xx-xx-xx | Modifications | name)
2024-11-11 | users table creation | sorryu
2024-11-17 | Add phone number | sorryu

*/

CREATE TABLE users {
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    number VARCHAR(20) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
}