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
mod filters;
mod happy_case;
mod health_check;

const ACTIVE_DATABASE: &'static str = "books.db";
const BACKUP_DATABASE: &'static str = "books.backup.db";
const EMPTY_DATABASE: &'static str = "empty.db";

struct TestClient {
    client: Client,
}
impl TestClient {
    fn setup_empty() -> Self {
        rename(ACTIVE_DATABASE, BACKUP_DATABASE).expect("Backup production database");
        copy(EMPTY_DATABASE, ACTIVE_DATABASE).expect("Copy empty database");
        Self {
            client: Client::tracked(rocket()).expect("Valid rocket instance"),
        }
    }

    fn setup_with_books() -> Self {
        let client = Self::setup_empty();
        client.add_books(vec![
            NewBook {
                title: "Introduction to Algorithms".into(),
                author: "Thomas H. Cormen".into(),
                year: 1989,
                publisher: Some("MIT Press".into()),
                description: Some("Original printing".into()),
            },
            NewBook {
                title: "Introduction to Algorithms".into(),
                author: "Thomas H. Cormen".into(),
                year: 2022,
                publisher: Some("MIT Press".into()),
                description: Some("Fourth edition".into()),
            },
            NewBook {
                title: "Clean Code".into(),
                author: "Robert C. Martin".into(),
                year: 2008,
                publisher: Some("Pearson".into()),
                description: Some("Much java, such wow".into()),
            },
            NewBook {
                title: "Refactoring: Improving the Design of Existing Code".into(),
                author: "Martin Fowler".into(),
                year: 2019,
                publisher: Some("Addison-Wesley Professional".into()),
                description: Some("Second edition".into()),
            },
            NewBook {
                title: "Refactoring: Improving the Design of Existing Code".into(),
                author: "Martin Fowler".into(),
                year: 1999,
                publisher: Some("Addison-Wesley Professional".into()),
                description: Some("First edition".into()),
            },
            NewBook {
                title: "Pragmatic Programmer, The: Your journey to mastery".into(),
                author: "David Thomas".into(),
                year: 2019,
                publisher: Some("Addison-Wesley Professional".into()),
                description: Some("20th Anniversary Edition".into()),
            },
            NewBook {
                title: "Pragmatic Programmer, The: Your journey to mastery".into(),
                author: "David Thomas".into(),
                year: 1999,
                publisher: Some("Addison-Wesley Professional".into()),
                description: Some("First edition".into()),
            },
        ]);
        client
    }

    fn teardown(self) {
        drop(self.client);
        remove_file(ACTIVE_DATABASE).expect("Delete test database after use");
        rename(BACKUP_DATABASE, ACTIVE_DATABASE).expect("Replace active database");
    }

    fn add_books(&self, books: Vec<NewBook>) {
        for (index, book) in books.into_iter().enumerate() {
            self.add_book(book, (index + 1) as i32);
        }
    }

    fn check_book_list(&self, expected: Vec<Book>) {
        self.check_book_list_expect_status(Some(expected), Status::Ok);
    }

    fn check_book_list_expect_status(&self, expected: Option<Vec<Book>>, status: Status) {
        let response = Self::make_request_expect_status(self.client.get("/books"), status);
        assert_eq!(response.into_json::<Vec<Book>>(), expected);
    }

    fn get_all_books(&self) -> Vec<Book> {
        self.get_book_list("".into())
    }

    fn get_book_list(&self, query_params: String) -> Vec<Book> {
        let response = Self::make_request_expect_status(
            self.client.get(format!("/books?{}", query_params)),
            Status::Ok,
        );
        response.into_json().unwrap()
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

    fn get_expect_status(&self, url: String, status: Status) -> LocalResponse {
        Self::make_request_expect_status(self.client.get(url), status)
    }

    fn make_request_expect_status(request: LocalRequest, status: Status) -> LocalResponse {
        let response = request.dispatch();
        assert_eq!(response.status(), status);
        response
    }
}
