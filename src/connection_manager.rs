//!This module manages the connection to the postgres database

use dotenv::from_path;
use std::env::var;
use std::env::VarError;

///Specifies the environment variable name for the database's user.
const VAR_USER: &str = "DB_USER";
///Specifies the environment variable name for the database's password.
const VAR_PASSWORD: &str = "DB_PASSWORD";
///Specifies the environment variable name for the database's host.
const VAR_HOST: &str = "DB_HOST";
///Specifies the environment variable name for the database's name.
const VAR_NAME: &str = "DB_NAME";
///Specifies the environment variable name for the maximum number of active connections for the database.
const VAR_MAX_CONNECTIONS: &str = "DB_MAX_CONNECTIONS";
///Specifies the environment variable name for the timeout seconds for the requests.
const VAR_CONNECTION_TIMEOUT: &str = "DB_CONNECTION_TIMEOUT";
///Specifies the environment variable name for the idle timeout on database sessions.
const VAR_IDLE_TIMEOUT: &str = "DB_IDLE_TIMEOUT";
///Specifies the environment variable name for the minimum amount of connections to maintain at all times.
const VAR_MIN_CONNECTIONS: &str = "DB_MIN_CONNECTIONS";

///Specifies the path for the environment variables file
pub const ENV_PATH: &str = ".env";

fn get_variable_name_by_index(index: &usize) -> &str {
    //!It obtains the variable name given an index and following an order.
    match index {
        0 => VAR_USER,
        1 => VAR_PASSWORD,
        2 => VAR_HOST,
        3 => VAR_NAME,
        _ => "NONE",
    }
}
pub fn get_url_envs(
    path: &str,
) -> (
    Result<String, VarError>,
    Result<String, VarError>,
    Result<String, VarError>,
    Result<String, VarError>,
) {
    //!It obtains from a path the four environment variables to build the url, or an error.
    match from_path(path) {
        Ok(()) => {
            return (
                var(VAR_USER),
                var(VAR_PASSWORD),
                var(VAR_HOST),
                var(VAR_NAME),
            );
        }
        Err(e) => {
            println!("Couldn't find the environment variables file: {}", e);
            return (
                var(VAR_USER),
                var(VAR_PASSWORD),
                var(VAR_HOST),
                var(VAR_NAME),
            );
        }
    }
}

fn get_connection_config(
    user: String,
    password: String,
    host: String,
    name: String,
) -> rocket_db_pools::Config {
    //!It builds the configuration for the database connection
    rocket_db_pools::Config {
        max_connections: 24,
        url: format!("postgres://{user}:{password}@{host}/{name}"),
        connect_timeout: 5,
        min_connections: None,
        idle_timeout: None,
    }
}

pub fn connect(
    data: (
        Result<String, VarError>,
        Result<String, VarError>,
        Result<String, VarError>,
        Result<String, VarError>,
    ),
) -> rocket_db_pools::Config {
    //!It returns the client of a postgres database or an error given the tuple from the function get_envs
    let mut credentials: Vec<String> = vec![];
    for (i, d) in [data.0, data.1, data.2, data.3].iter().enumerate() {
        match d {
            Ok(c) => credentials.push(c.to_string()),
            Err(e) => {
                println!(
                    "An error has ocurred while obtaining variable {}: {e}",
                    get_variable_name_by_index(&i)
                );
                credentials.push(format!(""));
            }
        }
    }
    get_connection_config(
        credentials.get(0).unwrap().to_string(),
        credentials.get(1).unwrap().to_string(),
        credentials.get(2).unwrap().to_string(),
        credentials.get(3).unwrap().to_string(),
    )
}
