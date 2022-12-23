#[macro_use]
extern crate diesel;

use rocket::launch;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

mod api_validation;
mod database;
mod endpoints;
mod schema;
#[cfg(test)]
mod tests;
mod types;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(database::Db::fairing())
        .mount("/", endpoints::endpoints())
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
}
