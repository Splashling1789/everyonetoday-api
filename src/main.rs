mod connection_manager;
mod routes;
#[macro_use] extern crate rocket;

use std::env;
use std::error::Error;
use std::ops::{Deref, DerefMut};
use std::sync::Mutex;
use rocket_db_pools::{Database, sqlx};
use crate::connection_manager::get_envs;
use routes::*;



struct quote_post {
    quote:String,
    sign:String,
}

#[launch]
fn rocket() -> _ {
    println!("{:?}", env::current_dir().unwrap());
    let conn = connection_manager::connect(get_envs(".env"));
    println!("Config: {:?}", conn);
    let figment = rocket::Config::figment().merge(("databases.main_db", conn));
    //let conn_state = AppState {db_client: Mutex::new(conn)};
    rocket::custom(figment).attach(MainDb::init()).mount("/", routes![health])
}