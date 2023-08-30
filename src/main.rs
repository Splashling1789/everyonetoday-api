mod connection_manager;
mod routes;
#[macro_use]
extern crate rocket;

use crate::connection_manager::{get_url_envs, ENV_PATH};
use rocket_db_pools::Database;
use routes::*;
use std::env;

struct quote_post {
    quote: String,
    sign: String,
}

#[launch]
fn rocket() -> _ {
    println!("{:?}", env::current_dir().unwrap());
    let conn = connection_manager::connect(get_url_envs(ENV_PATH));
    println!("Config: {:?}", conn);
    let figment = rocket::Config::figment().merge(("databases.main_db", conn));
    rocket::custom(figment)
        .attach(MainDb::init())
        .mount("/", routes![health])
}
