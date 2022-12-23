use crate::types::{Book, NewBook};

use super::TestClient;

#[test]
fn happy_case() {
    let client = TestClient::setup_empty();

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

#[test]
fn delete_one_of_many() {
    let client = TestClient::setup_with_books();

    client.delete_book(5);

    let all_books = client.get_all_books();
    assert!(!all_books.contains(&Book {
        id: Some(5),
        title: "Refactoring: Improving the Design of Existing Code".into(),
        author: "Martin Fowler".into(),
        year: 1999,
        publisher: Some("Addison-Wesley Professional".into()),
        description: Some("First edition".into()),
    }));

    // Indices of the remaining books didn't get adjusted
    assert!(all_books.contains(&Book {
        id: Some(7),
        title: "Pragmatic Programmer, The: Your journey to mastery".into(),
        author: "David Thomas".into(),
        year: 1999,
        publisher: Some("Addison-Wesley Professional".into()),
        description: Some("First edition".into()),
    }));

    client.teardown();
}
