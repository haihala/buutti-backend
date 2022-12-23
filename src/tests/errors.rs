use rocket::http::Status;

use crate::types::{Book, NewBook};

use super::TestClient;

#[test]
fn get_nonexistent_book() {
    let client = TestClient::setup_empty();
    client.check_book_expect_status(1, None, Status::NotFound);
    client.teardown();
}

#[test]
fn delete_nonexistent_book() {
    let client = TestClient::setup_empty();
    client.delete_book_expect_status(1, Status::NotFound);
    client.teardown();
}

#[test]
fn add_duplicate_book() {
    let client = TestClient::setup_empty();

    let book_to_add = NewBook {
        title: "Clean Code".into(),
        author: "Robert C. Martin".into(),
        year: 2008,
        publisher: Some("Pearson".into()),
        description: Some("Much java, such wow".into()),
    };
    client.add_book(book_to_add.clone(), 1);
    client.add_book_expect_status(book_to_add.clone(), None, Status::BadRequest);

    let mut added_book = Book::from(book_to_add);
    added_book.id = Some(1);
    client.check_book_list(vec![added_book]);

    client.teardown();
}
