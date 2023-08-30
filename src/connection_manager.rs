//!This module manages the connection to the postgres database

use std::env::VarError;
use std::env::var;
use dotenv::{from_path};
use rocket_db_pools::{Pool, sqlx};

///Specifies the variable name for the database's user
const VAR_USER:&str = "DB_USER";
///Specifies the variable name for the database's password
const VAR_PASSWORD:&str = "DB_PASSWORD";
///Specifies the variable name for the database's host
const VAR_HOST:&str = "DB_HOST";
///Specifies the variable name for the database's name
const VAR_NAME:&str = "DB_NAME";

fn get_variable_name_by_index(index:&usize) -> &str {
    //!It obtains the variable name given an index and following an order.
    match index {
        0 => {VAR_USER}
        1 => {VAR_PASSWORD}
        2 => {VAR_HOST}
        3 => {VAR_NAME}
        _ => {"NONE"}
    }
}
pub fn get_envs(path:&str) -> (Result<String, VarError>, Result<String, VarError>, Result<String, VarError>, Result<String, VarError>) {
    //!It obtains from a path the four environment variables or an error.
    match from_path(path) {
        Ok(()) => {
            return (var(VAR_USER), var(VAR_PASSWORD), var(VAR_HOST), var(VAR_NAME));
        }
        Err(e) => {
            println!("Couldn't find the environment variables file: {}", e);
            return (var(VAR_USER), var(VAR_PASSWORD), var(VAR_HOST), var(VAR_NAME));
        }
    }
}

fn get_connection (user: String, password: String, host:String, name: String) -> rocket_db_pools::Config {
    //!It connects to a database given user, password, host and database name
    rocket_db_pools::Config {
        max_connections: 24,
        url: format!("postgres://{user}:{password}@{host}/{name}"),
        connect_timeout: 5,
        min_connections: None,
        idle_timeout: None
    }
}

pub fn connect(data: (Result<String, VarError>, Result<String, VarError>, Result<String, VarError>, Result<String, VarError>)) -> rocket_db_pools::Config {
    //!It returns the client of a postgres database or an error given the tuple from the function get_envs
    let mut credentials: Vec<String> = vec![];
    for (i, d) in [data.0, data.1, data.2, data.3].iter().enumerate() {
        match d {
            Ok(c) => {credentials.push(c.to_string())}
            Err(e) => {
                println!("An error has ocurred while obtaining variable {}: {e}", get_variable_name_by_index(&i));
                credentials.push(format!(""));
            }
        }
    }
    get_connection(
        credentials.get(0).unwrap().to_string(),
        credentials.get(1).unwrap().to_string(),
        credentials.get(2).unwrap().to_string(),
        credentials.get(3).unwrap().to_string()
    )
}