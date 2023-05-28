// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
        descrip -> Varchar,
        author -> Varchar,
        cover_image -> Varchar,
        price -> Varchar,
    }
}
