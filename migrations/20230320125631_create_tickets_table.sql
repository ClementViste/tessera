-- Create `tickets` table.
CREATE TABLE tickets(
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL
);