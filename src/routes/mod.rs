use std::ops::Deref;
use rocket::State;
use std::sync::Mutex;
use postgres::{Client};


pub struct AppState {
    pub db_client: Mutex<Result<Client, postgres::Error>>,
}


#[get("/health")]
pub fn health(state: &State<AppState>) -> String {
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