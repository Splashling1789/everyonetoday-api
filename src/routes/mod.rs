use std::ops::Deref;
use rocket::State;
use std::sync::Mutex;
use postgres::{Client};
use rocket_db_pools::{Connection, Database, sqlx};

#[derive(Database)]
#[database("main_db")]
pub struct MainDb(sqlx::PgPool);

#[get("/health")]
pub async fn health(mut db: Connection<MainDb>) -> String {
    let mut db_status = String::default();
    match sqlx::query("SELECT * FROM quotes LIMIT 1;").fetch_one(&mut *db).await {
        Ok(_) => {
            db_status = format!("db_connection: \"OK\"");
        }
        Err(e) => {
            db_status = format!("db_connection: \"{}\"", e);
        }
    }
    format!("status: \"OK\"\n{db_status}")
}