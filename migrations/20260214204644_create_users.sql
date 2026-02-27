CREATE TABLE users (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    application TEXT DEFAULT 'AURA',
    password TEXT NOT NULL,
    UNIQUE(email, application)
);