#[macro_use]
extern crate diesel;

extern crate dotenv;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;

mod schema;
mod models;


fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = PgConnection::establish(&database_url).unwrap();

    let book = models::NewBook {
        title: String::from("Gravity's Rainbow"),
        author: String::from("Thomas Pynchon"),
        descrip: String::from("didididi daba"),
        cover_image:String::from("some_link"),
        price:String::from("1000")
    };

    if models::Book::insert(book, &conn) {
        println!("success");
    } else {
        println!("failed");
    }
}
