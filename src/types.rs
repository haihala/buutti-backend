use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Book {
    title: String,
    author: String,
    year: u32,
    publisher: Option<String>,
    description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BookId(pub u32);

#[derive(Serialize, Deserialize)]
pub struct ApiException {
    message: String,
}
impl ApiException {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
