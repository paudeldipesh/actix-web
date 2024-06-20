use actix_web::{get, middleware::Logger, web, App, HttpServer, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv::dotenv().ok();
    env_logger::init();

    HttpServer::new(|| App::new().wrap(Logger::default()).service(greet))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
