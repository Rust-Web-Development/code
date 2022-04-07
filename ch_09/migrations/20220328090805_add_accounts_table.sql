CREATE TABLE IF NOT EXISTS accounts (
    id uuid DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL PRIMARY KEY,
    password VARCHAR(255) NOT NULL
);
