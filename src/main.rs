mod connection_manager;
mod routes;
#[macro_use] extern crate rocket;

use std::env;
use std::error::Error;
use std::ops::{Deref, DerefMut};
use std::sync::Mutex;
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
    let conn_state = AppState {db_client: Mutex::new(conn)};
    rocket::build().manage(conn_state).mount("/", routes![health])
}