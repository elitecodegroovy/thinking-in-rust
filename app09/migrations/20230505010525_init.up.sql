-- Add up migration script here
CREATE TABLE
    IF NOT EXISTS notes (
    id  serial primary key,
    title VARCHAR(255) NOT NULL UNIQUE,
    content TEXT NOT NULL,
    category VARCHAR(100),
    published BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP
    WITH
        TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP
    WITH
        TIME ZONE DEFAULT NOW()
    );