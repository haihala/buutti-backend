use rocket_sync_db_pools::database;

use crate::types::{ApiBook, BookId, ORMBook};

#[database("books_db")]
pub struct Db(diesel::SqliteConnection);

pub struct NonexistentBook {
    pub book_id: BookId,
}

pub fn store_book(connection: Db, book: ApiBook) -> BookId {
    todo!()
}

pub fn get_books(connection: Db) -> Vec<ORMBook> {
    todo!()
}

pub fn get_book(connection: Db, id: BookId) -> Result<ORMBook, NonexistentBook> {
    todo!()
}

pub fn delete_book(connection: Db, id: BookId) -> Result<ORMBook, NonexistentBook> {
    todo!()
}
