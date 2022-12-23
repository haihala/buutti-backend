#[macro_use]
extern crate diesel;

use rocket::{launch, routes};

mod api_validation;
mod database;
mod endpoints;
mod schema;
#[cfg(test)]
mod tests;
mod types;

use database::Db;
use endpoints::book_routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::fairing())
        .mount("/books", book_routes())
        .mount("/health-check", routes![endpoints::health_check])
}
