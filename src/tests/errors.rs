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

#[test]
fn invalid_filters() {
    let client = TestClient::setup_with_books();

    // How additional unknown query filters should be handled wasn't specified, so they are ignored
    let all_books = client.get_book_list_expect_status("unknown=value".into(), Status::Ok);
    assert_eq!(all_books.len(), 7);

    // Empty strings for title and author are not tolerated
    client.get_book_list_expect_status("author=".into(), Status::BadRequest);
    client.get_book_list_expect_status("title=".into(), Status::BadRequest);

    // If the value cannot be parsed to an integer, it is treated as if it didn't exist
    // This may be against the specifications, but it's how rocket handles things by default
    // This can be fixed, if it is.
    assert_eq!(all_books, client.get_book_list("year=".into()));

    client.teardown();
}
