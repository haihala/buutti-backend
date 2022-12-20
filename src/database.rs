use crate::types::{Book, BookId};

pub struct NonexistentBook {
    pub book_id: BookId,
}

pub fn store_book() -> BookId {
    todo!()
}

pub fn get_books() -> Vec<Book> {
    todo!()
}

pub fn get_book(id: BookId) -> Result<Book, NonexistentBook> {
    todo!()
}

pub fn delete_book(id: BookId) -> Result<Book, NonexistentBook> {
    todo!()
}
