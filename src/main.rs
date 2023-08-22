mod connection_manager;

#[macro_use] extern crate rocket;

use std::env;
use std::error::Error;
use std::ops::{Deref, DerefMut};
use std::sync::Mutex;
use rocket::State;
use postgres::{Client};
use crate::connection_manager::get_envs;

struct AppState {
    db_client: Mutex<Result<Client, postgres::Error>>,
}


struct quote_post {
    quote:String,
    sign:String,
}

#[get("/health")]
fn health(state: &State<AppState>) -> String {
    let mut db_status = String::default();
    match state.db_client.lock().unwrap().deref() {
        Ok(_) => {
            db_status = format!("db_connection: \"OK\"");
        }
        Err(e) => {
            db_status = format!("db_connection: \"{}\"", e);
        }
    }
    format!("status: \"OK\"\n{db_status}")
}

#[launch]
fn rocket() -> _ {
    println!("{:?}", env::current_dir().unwrap());
    let conn = connection_manager::connect(get_envs(".env"));
    let conn_state = AppState {db_client: Mutex::new(conn)};
    rocket::build().manage(conn_state).mount("/", routes![health])
}