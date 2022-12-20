use diesel::{dsl::max, QueryDsl, RunQueryDsl};
use rocket_sync_db_pools::database;

use crate::{
    schema::books::{self, id},
    types::{ApiBook, BookId, ORMBook},
};

#[database("books_db")]
pub struct Db(diesel::SqliteConnection);

pub struct NonexistentBook {
    pub book_id: BookId,
}

pub async fn store_book(connection: Db, book: ApiBook) -> BookId {
    connection
        .run(|c| {
            diesel::insert_into(books::table)
                .values(ORMBook::from(book))
                .execute(c)
                .unwrap();

            books::table.select(max(id)).execute(c)
        })
        .await
        .map(|val| BookId(val as i32))
        .expect("Failed to get a book id after insert")
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

pub fn delete_book(connection: Db, book_id: BookId) -> Result<ORMBook, NonexistentBook> {
    todo!()
}
