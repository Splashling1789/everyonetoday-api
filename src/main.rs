//# Everyone Today: A wall for everyone to leave some words anonymously
pub const API_VERSION: &str = env!("CARGO_PKG_VERSION");

mod connection_manager;
mod routes;
#[macro_use]
extern crate rocket;

use rocket_db_pools::Database;
use routes::*;

#[launch]
fn rocket() -> _ {
    let conn = connection_manager::get_connection_config();
    let figment = rocket::Config::figment().merge(("databases.main_db", conn));
    rocket::custom(figment)
        .attach(MainDb::init())
        .mount(
            "/",
            routes![get_health::health, get_posts::posts, post_write::write],
        )
        .register("/", catchers![get_health::not_available])
}
