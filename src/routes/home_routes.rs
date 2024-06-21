use super::handlers;
use actix_web::web;

pub fn configuration(configure: &mut web::ServiceConfig) {
    configure.service(
        web::scope("/api")
            .service(handlers::home_handlers::greet)
            .service(handlers::home_handlers::home),
    );
}
