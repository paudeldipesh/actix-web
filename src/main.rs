use actix_web::{middleware::Logger, web, App, HttpServer};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use utils::app_status::AppState;
mod routes;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv::dotenv().ok();
    env_logger::init();

    let address = (*utils::constants::ADDRESS).clone();
    let port = (*utils::constants::PORT).clone();
    let database_url = (*utils::constants::DATABASE_URL).clone();

    // Connect to the database
    let db = match Database::connect(database_url).await {
        Ok(db) => db,
        Err(err) => {
            eprintln!("Failed to connect to the database: {}", err);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Database connection error",
            ));
        }
    };

    // Run migrations
    match Migrator::up(&db, None).await {
        Ok(_) => println!("Database migration successful"),
        Err(err) => {
            eprintln!("Failed to run migrations: {}", err);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Migration error",
            ));
        }
    };

    println!("Server running at http://{}:{}", address, port);

    // Start Actix Web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .wrap(Logger::default())
            .configure(routes::home_routes::configuration)
            .configure(routes::auth_routes::configuration)
    })
    .bind((address, port))?
    .run()
    .await
}
