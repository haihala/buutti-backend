use crate::types::{Book, NewBook};

use super::TestClient;

#[test]
fn happy_case() {
    let client = TestClient::setup();

    // Check that the book list starts empty
    client.check_book_list(vec![]);

    // Add a new book
    let book_to_add = NewBook {
        title: "Clean Code".into(),
        author: "Robert C. Martin".into(),
        year: 2008,
        publisher: Some("Pearson".into()),
        description: Some("Much java, such wow".into()),
    };
    client.add_book(book_to_add.clone(), 1);

    // Check if book got added
    let mut added_book = Book::from(book_to_add);
    added_book.id = Some(1);
    client.check_book_list(vec![added_book.clone()]);
    client.check_book(1, added_book);

    client.delete_book(1);

    // Check if book got deleted
    client.check_book_list(vec![]);
    client.teardown();
}
