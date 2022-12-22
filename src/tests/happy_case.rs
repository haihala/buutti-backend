use rocket::{
    http::Status,
    local::blocking::{Client, LocalRequest, LocalResponse},
};

use crate::types::{Book, BookId, NewBook};

use super::{setup, teardown};

#[test]
fn happy_case() {
    let client = setup();

    // Check that the book list starts empty
    check_book_list(&client, vec![]);

    // Add a new book
    let book_to_add = NewBook {
        title: "Clean Code".into(),
        author: "Robert C. Martin".into(),
        year: 2008,
        publisher: Some("Pearson".into()),
        description: Some("Much java, such wow".into()),
    };
    add_book(&client, book_to_add.clone(), 1);

    // Check if book got added
    let mut added_book = Book::from(book_to_add);
    added_book.id = Some(1);
    check_book_list(&client, vec![added_book.clone()]);
    check_book(&client, 1, added_book);

    delete_book(&client, 1);

    // Check if book got deleted
    check_book_list(&client, vec![]);
    teardown(client);
}

fn check_book_list(client: &Client, expected: Vec<Book>) {
    let response = make_request(client.get("/books"));
    assert_eq!(response.into_json::<Vec<Book>>().unwrap(), expected);
}

fn check_book(client: &Client, id: i32, expected: Book) {
    let response = make_request(client.get(format!("/books/{}", id)));
    assert_eq!(response.into_json::<Book>().unwrap(), expected);
}

fn add_book(client: &Client, new_book: NewBook, expected_id: i32) {
    let response = make_request(
        client
            .post("/books")
            .body(serde_json::to_string(&new_book).expect("Failed to serialize")),
    );

    assert_eq!(
        response.into_json::<BookId>().unwrap(),
        BookId::new(expected_id)
    );
}

fn delete_book(client: &Client, id: i32) {
    make_request_expect_status(client.delete(format!("/books/{}", id)), Status::NoContent);
}

fn make_request(request: LocalRequest) -> LocalResponse {
    make_request_expect_status(request, Status::Ok)
}

fn make_request_expect_status(request: LocalRequest, status: Status) -> LocalResponse {
    let response = request.dispatch();
    assert_eq!(response.status(), status);

    response
}
