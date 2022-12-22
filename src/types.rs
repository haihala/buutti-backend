use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::books;
#[derive(Queryable, Debug, Insertable, Serialize, Clone)]
#[table_name = "books"]
pub struct Book {
    id: Option<i32>,
    pub title: String,
    pub author: String,
    pub year: i32,
    publisher: Option<String>,
    description: Option<String>,
}
impl From<NewBook> for Book {
    fn from(value: NewBook) -> Self {
        Self {
            id: None,
            title: value.title,
            author: value.author,
            year: value.year,
            publisher: value.publisher,
            description: value.description,
        }
    }
}

#[derive(Deserialize, Clone)]
/// Struct for creating new books so that the API gets typed correctly
pub struct NewBook {
    pub title: String,
    pub author: String,
    pub year: i32,
    publisher: Option<String>,
    description: Option<String>,
}
impl From<Book> for NewBook {
    fn from(orm_book: Book) -> Self {
        Self {
            title: orm_book.title,
            author: orm_book.author,
            year: orm_book.year,
            publisher: orm_book.publisher,
            description: orm_book.description,
        }
    }
}

#[derive(Serialize, Clone, Copy)]
pub struct BookId {
    pub id: i32,
}
impl BookId {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}
