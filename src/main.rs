use actix_web::{middleware::Logger, web, App, HttpServer};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::{fmt::Display, io::Error};
use utils::app_status::AppState;
mod routes;
mod utils;

#[derive(Debug)]
#[allow(dead_code)]
struct MainError {
    message: String,
}

impl Display for MainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

#[actix_web::main]
async fn main() -> Result<(), MainError> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv::dotenv().ok();
    env_logger::init();

    let address: String = (*utils::constants::ADDRESS).clone();
    let port: u16 = (*utils::constants::PORT).clone();
    let database_url: String = (*utils::constants::DATABASE_URL).clone();

    let db: DatabaseConnection = Database::connect(database_url)
        .await
        .map_err(|err: DbErr| MainError {
            message: err.to_string(),
        })?;

    Migrator::up(&db, None)
        .await
        .map_err(|err: DbErr| MainError {
            message: err.to_string(),
        })?;

    println!("Server running at http://{}:{}", address, port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .wrap(Logger::default())
            .configure(routes::home_routes::configuration)
            .configure(routes::auth_routes::configuration)
            .configure(routes::user_routes::configuration)
            .configure(routes::post_routes::configuration)
    })
    .bind((address, port))
    .map_err(|err: Error| MainError {
        message: err.to_string(),
    })?
    .run()
    .await
    .map_err(|err: Error| MainError {
        message: err.to_string(),
    })
}

/*
Generate entity files: sea-orm-cli generate entity -o entity/src
Continuous build watcher: cargo watch -x run
*/
