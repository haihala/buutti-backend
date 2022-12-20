use diesel::{QueryDsl, RunQueryDsl};
use rocket_sync_db_pools::database;

use crate::{
    schema::books,
    types::{ApiBook, BookId, ORMBook},
};

#[database("books_db")]
pub struct Db(diesel::SqliteConnection);

pub struct NonexistentBook {
    pub book_id: BookId,
}

pub fn store_book(connection: Db, book: ApiBook) -> BookId {
    todo!()
}

pub async fn get_books(connection: Db) -> Vec<ORMBook> {
    connection
        .run(|c| books::table.load(c))
        .await
        .expect("Failed to fetch books")
}

pub async fn get_book(connection: Db, book_id: BookId) -> Result<ORMBook, NonexistentBook> {
    connection
        .run(move |c| books::table.find(book_id.0 as i32).first(c))
        .await
        .map_err(|_| NonexistentBook { book_id })
}

pub fn delete_book(connection: Db, id: BookId) -> Result<ORMBook, NonexistentBook> {
    todo!()
}
