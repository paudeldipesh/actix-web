use super::handlers;
use actix_web::web;

pub fn configuration(configure: &mut web::ServiceConfig) {
    configure.service(
        web::scope("/auth")
            .service(handlers::auth_handlers::register)
            .service(handlers::auth_handlers::login)
            .service(handlers::auth_handlers::hi),
    );
}
