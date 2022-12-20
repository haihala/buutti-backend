#[macro_use]
extern crate diesel;

use rocket::launch;

mod database;
mod endpoints;
mod schema;
mod types;

use database::Db;
use endpoints::book_routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::fairing())
        .mount("/books", book_routes())
}
