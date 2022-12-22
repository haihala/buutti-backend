use rocket::{
    http::Status,
    local::blocking::{Client, LocalRequest, LocalResponse},
};
use std::fs::{copy, remove_file, rename};

use super::{
    rocket,
    types::{Book, BookId, NewBook},
};
mod errors;
mod happy_case;

const ACTIVE_DATABASE: &'static str = "books.db";
const BACKUP_DATABASE: &'static str = "books.backup.db";
const EMPTY_DATABASE: &'static str = "empty.db";

struct TestClient {
    client: Client,
}
impl TestClient {
    fn setup() -> Self {
        rename(ACTIVE_DATABASE, BACKUP_DATABASE).expect("Backup production database");
        copy(EMPTY_DATABASE, ACTIVE_DATABASE).expect("Copy empty database");
        Self {
            client: Client::tracked(rocket()).expect("Valid rocket instance"),
        }
    }

    fn teardown(self) {
        drop(self.client);
        remove_file(ACTIVE_DATABASE).expect("Delete test database after use");
        rename(BACKUP_DATABASE, ACTIVE_DATABASE).expect("Replace active database");
    }

    fn check_book_list(&self, expected: Vec<Book>) {
        self.check_book_list_expect_status(Some(expected), Status::Ok);
    }

    fn check_book_list_expect_status(&self, expected: Option<Vec<Book>>, status: Status) {
        let response = Self::make_request_expect_status(self.client.get("/books"), status);
        assert_eq!(response.into_json::<Vec<Book>>(), expected);
    }

    fn check_book(&self, id: i32, expected: Book) {
        self.check_book_expect_status(id, Some(expected), Status::Ok)
    }

    fn check_book_expect_status(&self, id: i32, expected: Option<Book>, status: Status) {
        let response =
            Self::make_request_expect_status(self.client.get(format!("/books/{}", id)), status);
        assert_eq!(response.into_json::<Book>(), expected);
    }

    fn add_book(&self, new_book: NewBook, expected_id: i32) {
        self.add_book_expect_status(new_book, Some(expected_id), Status::Ok)
    }

    fn add_book_expect_status(&self, new_book: NewBook, expected_id: Option<i32>, status: Status) {
        let response = Self::make_request_expect_status(
            self.client
                .post("/books")
                .body(serde_json::to_string(&new_book).expect("Failed to serialize")),
            status,
        );

        assert_eq!(response.into_json::<BookId>(), expected_id.map(BookId::new));
    }

    fn delete_book(&self, id: i32) {
        self.delete_book_expect_status(id, Status::NoContent);
    }

    fn delete_book_expect_status(&self, id: i32, status: Status) {
        Self::make_request_expect_status(self.client.delete(format!("/books/{}", id)), status);
    }

    fn make_request_expect_status(request: LocalRequest, status: Status) -> LocalResponse {
        let response = request.dispatch();
        assert_eq!(response.status(), status);
        response
    }
}
