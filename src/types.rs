use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::books;
#[derive(Queryable, Debug, Insertable)]
#[table_name = "books"]
pub struct ORMBook {
    id: i32,
    title: String,
    author: String,
    year: i32,
    publisher: Option<String>,
    description: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct ApiBook {
    title: String,
    author: String,
    year: u32,
    publisher: Option<String>,
    description: Option<String>,
}
impl From<ORMBook> for ApiBook {
    fn from(orm_book: ORMBook) -> Self {
        Self {
            title: orm_book.title,
            author: orm_book.author,
            year: orm_book.year as u32,
            publisher: orm_book.publisher,
            description: orm_book.description,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct BookId(pub u32);

#[derive(Serialize, Deserialize)]
pub struct ApiException {
    message: String,
}
impl ApiException {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
