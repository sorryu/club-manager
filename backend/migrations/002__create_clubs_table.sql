-- Title: create_clubs_table
-- Created by sorryu
-- Date: 2024-11-11
-- Description: Clubs table creation migration file.

/*
History(ex: 20xx-xx-xx | Modifications | name)
2024-11-11 | clubs table creation | sorryu
2024-11-17 | Add foreign key, creation_userid | sorryu
2024-11-17 | Change from foreign key to References | sorryu

*/

CREATE TABLE clubs (
    id SERIAL PRIMARY KEY,
    creation_userid INT REFERENCES users(id), -- need to add foreign key connection
    name VARCHAR(100) UNIQUE NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)