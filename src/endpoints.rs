use rocket::http::Status;
use rocket::response::status::{BadRequest, NotFound};
use rocket::serde::json::Json;
use rocket::{delete, get, post, Route};
use rocket_okapi::{openapi, openapi_get_routes};

use crate::api_validation::{format_error_message, validate_non_empty, ApiException};
use crate::database;
use crate::types::{Book, BookId, NewBook};

pub fn endpoints() -> Vec<Route> {
    openapi_get_routes![post_book, get_books, get_book, delete_book, health_check]
}

#[openapi(tag = "books")]
#[post("/books", data = "<book>")]
async fn post_book(
    connection: database::Db,
    book: Json<NewBook>,
) -> Result<Json<BookId>, BadRequest<Json<ApiException>>> {
    database::store_book(connection, book.into_inner().into())
        .await
        .map(|id| Json(id))
        .map_err(|_| BadRequest(Some(Json(ApiException::new("Cannot add such book".into())))))
}

#[openapi(tag = "books")]
#[get("/books?<author>&<title>&<year>")]
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

#[openapi(tag = "books")]
#[get("/books/<id>")]
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

#[openapi(tag = "books")]
#[delete("/books/<id>")]
async fn delete_book(
    connection: database::Db,
    id: i32,
) -> Result<Status, NotFound<Json<ApiException>>> {
    database::delete_book(connection, BookId::new(id))
        .await
        .map(|_| Status::NoContent)
        .map_err(format_error_message)
}

#[openapi(tag = "Health-check")]
#[get("/health-check")]
fn health_check() -> String {
    "Ok".into()
}
