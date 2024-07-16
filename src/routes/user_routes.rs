use super::{handlers, middlewares};
use actix_web::web;
use actix_web_lab::middleware::from_fn;

pub fn configuration(configure: &mut web::ServiceConfig) {
    configure.service(
        web::scope("/user")
            .wrap(from_fn(
                middlewares::auth_middlewares::check_auth_middleware,
            ))
            .service(handlers::user_handlers::user),
    );
}
