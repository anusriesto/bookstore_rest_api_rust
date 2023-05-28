use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::schema::books;
use crate::schema::books::dsl::books as all_books;


#[derive(Queryable)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub descrip: String,
    pub author: String,
    pub cover_image: String,
    pub price:String,
}

#[derive(Insertable)]
#[diesel(table_name = books)]
pub struct NewBook {
    pub title: String,
    pub descrip:String,
    pub author: String,
    pub cover_image: String,
    pub price:String
}


impl Book {
    pub fn show(id: i32, conn: &PgConnection) -> Vec<Book> {
        all_books
            .find(id)
            .load::<Book>(conn)
            .expect("Error loading book")
    }

    pub fn all(conn: &PgConnection) -> Vec<Book> {
        all_books
            .order(books::id.desc())
            .load::<Book>(conn)
            .expect("error loading the books")
    }

    pub fn update_by_id(id: i32, conn: &PgConnection, book: NewBook) -> bool {
        use crate::schema::books::dsl::{author as a, descrip as d, title as t,cover_image as c,price as p};
        let NewBook {
            title,
            descrip,
            author,
            cover_image,
            price
        }  = book;

        diesel::update(all_books.find(id))
        .set((a.eq(author), d.eq(descrip), t.eq(title),c.eq(cover_image),p.eq(price)))
            .get_result::<Book>(conn)
            .is_ok()
    }

    pub fn insert(book: NewBook, conn: &PgConnection) -> bool {
        diesel::insert_into(books::table)
            .values(&book)
            .execute(conn)
            .is_ok()
    }

    pub fn delete_by_id(id: i32, conn: &PgConnection) -> bool {
        if Book::show(id, conn).is_empty() {
            return false;
        };
        diesel::delete(all_books.find(id)).execute(conn).is_ok()
    }

    pub fn all_by_author(author: String, conn: &PgConnection) -> Vec<Book> {
        all_books
            .filter(books::author.eq(author))
            .load::<Book>(conn)
            .expect("Error loading books by author")
    }
}


