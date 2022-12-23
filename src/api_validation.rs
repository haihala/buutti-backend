use rocket::http::Status;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket_okapi::okapi::schemars;
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::database;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ApiException {
    message: String,
}
impl ApiException {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

pub fn format_error_message(error: database::NonexistentBook) -> NotFound<Json<ApiException>> {
    NotFound(Json(ApiException::new(format!(
        "Book id {} not found",
        error.book_id.id
    ))))
}

pub fn validate_non_empty(input: &Option<String>) -> Result<(), Status> {
    if let Some(content) = input {
        if content.is_empty() {
            return Err(Status::BadRequest);
        }
    }

    Ok(())
}
