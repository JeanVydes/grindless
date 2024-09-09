use env::load_enviroment_vars;
use grindless_core::{db::sql::establish_postgres_connection, util::log::set_up_logger};
use server::init_server;

pub mod env;
pub mod server;
pub mod state;
pub mod routers;
pub mod controllers;
pub mod services;
pub mod util;

#[actix_web::main]
pub async fn main() {
    let enviroment = match load_enviroment_vars() {
        Ok(env) => env,
        Err(e) => {
            panic!("Error loading enviroment vars: {:?}", e);
        }
    };

    let logger_level = match enviroment.logger_level_filter.as_str() {
        "info" => log::LevelFilter::Info,
        "debug" => log::LevelFilter::Debug,
        "warn" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        _ => log::LevelFilter::Info,
    };

    match set_up_logger(logger_level) {
        Ok(_) => (),
        Err(e) => {
            panic!("Error setting up logger: {:?}", e);
        }
    }

    let postgres_conn = match establish_postgres_connection(&enviroment.postgres_url).await {
        Ok(conn) => conn,
        Err(e) => {
            panic!("Error establishing postgres connection: {:?}", e);
        }
    };

    match init_server(postgres_conn, enviroment).await {
        Ok(_) => (),
        Err(e) => {
            panic!("Error starting server: {:?}", e);
        }
    }
}