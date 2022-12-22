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

#[get("/?<author>&<title>&<year>")]
async fn get_books(
    connection: database::Db,
    author: Option<String>,
    title: Option<String>,
    year: Option<i32>,
) -> Result<Json<Vec<Book>>, Status> {
    validate_non_empty(&author)?;
    validate_non_empty(&title)?;
    Ok(Json(
        database::get_books(connection, author, title, year).await,
    ))
}

#[get("/<id>")]
async fn get_book(
    connection: database::Db,
    id: i32,
) -> Result<Json<Book>, NotFound<Json<ApiException>>> {
    Ok(Json(
        database::get_book(connection, BookId::new(id))
            .await
            .map_err(format_error_message)?,
    ))
}

#[delete("/<id>")]
async fn delete_book(
    connection: database::Db,
    id: i32,
) -> Result<Status, NotFound<Json<ApiException>>> {
    database::delete_book(connection, BookId::new(id))
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

fn validate_non_empty(input: &Option<String>) -> Result<(), Status> {
    if let Some(content) = input {
        if content.len() == 0 {
            return Err(Status::BadRequest);
        }
    }

    Ok(())
}
