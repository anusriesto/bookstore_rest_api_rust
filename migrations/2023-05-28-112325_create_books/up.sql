-- Your SQL goes here
CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    descrip VARCHAR NOT NULL,
    author VARCHAR NOT NULL,
    cover_image VARCHAR NOT NULL,
    price VARCHAR NOT NULL
)