use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::Route;

use crate::database;
use crate::types::{ApiException, Book, BookId};

#[post("/", data = "<book>")]
fn post_book(book: Json<Book>) -> Json<BookId> {
    let id = database::store_book();
    Json(id)
}

#[get("/")]
fn get_books() -> Json<Vec<Book>> {
    let db_books = database::get_books();
    Json(db_books)
}

#[get("/<id>")]
fn get_book(id: u32) -> Result<Json<Book>, NotFound<Json<ApiException>>> {
    let db_book = database::get_book(BookId(id)).map_err(format_error_message)?;
    Ok(Json(db_book))
}

#[delete("/<id>")]
fn delete_book(id: u32) -> Result<Json<Book>, NotFound<Json<ApiException>>> {
    let db_book = database::delete_book(BookId(id)).map_err(format_error_message)?;
    Ok(Json(db_book))
}

fn format_error_message(error: database::NonexistentBook) -> NotFound<Json<ApiException>> {
    NotFound(Json(ApiException::new(format!(
        "Book id {} not found",
        error.book_id.0
    ))))
}

pub fn book_routes() -> Vec<Route> {
    routes![post_book, get_books, get_book, delete_book]
}
