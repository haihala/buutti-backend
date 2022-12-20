#[macro_use]
extern crate rocket;

mod database;
mod endpoints;
mod types;

use endpoints::book_routes;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/books", book_routes())
}
