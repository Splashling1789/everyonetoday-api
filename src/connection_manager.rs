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
///Default max connections in case it is not specified.
const DEFAULT_MAX_CONNECTIONS: usize = 512;
///Specifies the environment variable name for the timeout seconds for the requests.
const VAR_CONNECTION_TIMEOUT: &str = "DB_CONNECTION_TIMEOUT";
///Default connection timeout in case it is not specified.
const DEFAULT_CONNECTION_TIMEOUT: u64 = 24;
///Specifies the environment variable name for the idle timeout on database sessions.
const VAR_IDLE_TIMEOUT: &str = "DB_IDLE_TIMEOUT";
///Default idle timeout in case it is not specified
const DEFAULT_IDLE_TIMEOUT: Option<u64> = Some(10);
///Specifies the environment variable name for the minimum amount of connections to maintain at all times.
const VAR_MIN_CONNECTIONS: &str = "DB_MIN_CONNECTIONS";
///Default min connections in case it is not specified
const DEFAULT_MIN_CONNECTIONS: Option<u32> = Some(5);

///Specifies the path for the environment variables file
pub const ENV_PATH: &str = ".env";

///Provides the components of the postgres url
struct UrlComponents {
    user: String,
    password: String,
    host: String,
    db_name: String,
}
impl UrlComponents {
    fn get_url(&self) -> String {
        //!Gets the postgres url based on the components of the struct
        format!(
            "postgres://{}:{}@{}/{}",
            self.user, self.password, self.host, self.db_name
        )
    }
}

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
fn get_url_envs(
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

fn build_url(
    components: (
        Result<String, VarError>,
        Result<String, VarError>,
        Result<String, VarError>,
        Result<String, VarError>,
    ),
) -> UrlComponents {
    //!It creates a secure UrlComponent given the results of the four environment variables that regards the url
    let mut credentials: Vec<String> = vec![];
    for (i, d) in [components.0, components.1, components.2, components.3]
        .iter()
        .enumerate()
    {
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
    UrlComponents {
        user: credentials.get(0).unwrap().to_string(),
        password: credentials.get(1).unwrap().to_string(),
        host: credentials.get(2).unwrap().to_string(),
        db_name: credentials.get(3).unwrap().to_string(),
    }
}
pub fn get_connection_config() -> rocket_db_pools::Config {
    //!It builds the configuration for the database connection
    let max_connections = match var(VAR_MAX_CONNECTIONS) {
        Ok(v) => match v.parse::<usize>() {
            Ok(r) => r,
            Err(_) => DEFAULT_MAX_CONNECTIONS,
        },
        Err(_) => DEFAULT_MAX_CONNECTIONS,
    };
    let connect_timeout = match var(VAR_CONNECTION_TIMEOUT) {
        Ok(v) => match v.parse::<u64>() {
            Ok(r) => r,
            Err(_) => DEFAULT_CONNECTION_TIMEOUT,
        },
        Err(_) => DEFAULT_CONNECTION_TIMEOUT,
    };
    let min_connections = match var(VAR_MIN_CONNECTIONS) {
        Ok(v) => match v.parse::<u32>() {
            Ok(r) => Some(r),
            Err(_) => DEFAULT_MIN_CONNECTIONS,
        },
        Err(_) => DEFAULT_MIN_CONNECTIONS,
    };
    let idle_timeout = match var(VAR_IDLE_TIMEOUT) {
        Ok(v) => match v.parse::<u64>() {
            Ok(r) => Some(r),
            Err(_) => DEFAULT_IDLE_TIMEOUT,
        },
        Err(_) => DEFAULT_IDLE_TIMEOUT,
    };
    rocket_db_pools::Config {
        max_connections: max_connections,
        url: build_url(get_url_envs(ENV_PATH)).get_url(),
        connect_timeout: connect_timeout,
        min_connections: min_connections,
        idle_timeout: idle_timeout,
        extensions: None,
    }
}
