use rocket::http::Status;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::{delete, get, post, routes, Route};

use crate::database;
use crate::types::{ApiException, Book, BookId, NewBook};

pub fn book_routes() -> Vec<Route> {
    routes![post_book, get_books, get_book, delete_book]
}

#[post("/", data = "<book>")]
async fn post_book(connection: database::Db, book: Json<NewBook>) -> Json<BookId> {
    Json(database::store_book(connection, book.into_inner().into()).await)
}

#[get("/")]
async fn get_books(connection: database::Db) -> Json<Vec<Book>> {
    Json(database::get_books(connection).await)
}

#[get("/<id>")]
async fn get_book(
    connection: database::Db,
    id: u32,
) -> Result<Json<Book>, NotFound<Json<ApiException>>> {
    Ok(Json(
        database::get_book(connection, BookId::new(id as i32))
            .await
            .map_err(format_error_message)?,
    ))
}

#[delete("/<id>")]
async fn delete_book(
    connection: database::Db,
    id: u32,
) -> Result<Status, NotFound<Json<ApiException>>> {
    database::delete_book(connection, BookId::new(id as i32))
        .await
        .map(|_| Status::NoContent)
        .map_err(format_error_message)
}

fn format_error_message(error: database::NonexistentBook) -> NotFound<Json<ApiException>> {
    NotFound(Json(ApiException::new(format!(
        "Book id {} not found",
        error.book_id.id
    ))))
}
