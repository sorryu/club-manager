-- Title: create_user_roles_table
-- Created by sorryu
-- Date: 2024-11-11
-- Description: User-club relationship table creation migration file.

/*
History(ex: 20xx-xx-xx | Modifications | name)
2024-11-11 | user_roles table creation | sorryu

*/

CREATE TABLE user_roles (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id),
    club_id INT REFERENCES clubs(id),
    role VARCHAR(50),
    UNIQUE (user_id, club_id)
)