use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket_sync_db_pools::database;

use crate::{
    schema::books::{self},
    types::{Book, BookId},
};

#[database("books_db")]
pub struct Db(diesel::SqliteConnection);

#[derive(Clone, Copy)]
pub struct NonexistentBook {
    pub book_id: BookId,
}

pub async fn store_book(connection: Db, book: Book) -> Result<BookId, diesel::result::Error> {
    connection
        .run(|c| {
            diesel::insert_into(books::table)
                .values(book.clone())
                .execute(c)?;

            // Since diesel sqlite doesn't support returning the id from the insertion, we have to find it otherhow
            // Since title+author+year combinations are unique, this is guaranteed to work
            books::table
                .filter(books::title.eq(book.title))
                .filter(books::author.eq(book.author))
                .filter(books::year.eq(book.year))
                .select(books::id)
                .first(c)
        })
        .await
        .map(|val: Option<i32>| BookId::new(val.unwrap()))
}

pub async fn get_books(
    connection: Db,
    query_author: Option<String>,
    query_title: Option<String>,
    query_year: Option<i32>,
) -> Vec<Book> {
    connection
        .run(move |c| {
            let mut query = books::table.into_boxed();
            if let Some(author) = query_author {
                query = query.filter(books::author.eq(author));
            }
            if let Some(title) = query_title {
                query = query.filter(books::title.eq(title));
            }
            if let Some(year) = query_year {
                query = query.filter(books::year.eq(year));
            }

            query.load(c)
        })
        .await
        .expect("Failed to fetch books")
}

pub async fn get_book(connection: Db, book_id: BookId) -> Result<Book, NonexistentBook> {
    connection
        .run(move |c| books::table.find(book_id.id).first(c))
        .await
        .map_err(|_| NonexistentBook { book_id })
}

pub async fn delete_book(connection: Db, book_id: BookId) -> Result<(), NonexistentBook> {
    let error = NonexistentBook { book_id };

    let lines_deleted = connection
        .run(move |c| diesel::delete(books::table.find(book_id.id)).execute(c))
        .await
        .map_err(|_| error)?;

    if lines_deleted == 0 {
        Err(error)
    } else {
        Ok(())
    }
}
