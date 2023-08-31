mod connection_manager;
mod routes;
#[macro_use]
extern crate rocket;

use rocket_db_pools::Database;
use routes::*;
use std::env;

struct quote_post {
    quote: String,
    sign: String,
}

#[launch]
fn rocket() -> _ {
    let conn = connection_manager::get_connection_config();
    let figment = rocket::Config::figment().merge(("databases.main_db", conn));
    rocket::custom(figment)
        .attach(MainDb::init())
        .mount("/", routes![health])
}
