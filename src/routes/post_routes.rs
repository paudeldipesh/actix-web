use super::{handlers, middlewares};
use actix_web::web;
use actix_web_lab::middleware::from_fn;

pub fn configuration(configure: &mut web::ServiceConfig) {
    configure
        .service(
            web::scope("secure/post")
                .wrap(from_fn(
                    middlewares::auth_middlewares::check_auth_middleware,
                ))
                .service(handlers::post_handlers::create_post)
                .service(handlers::post_handlers::my_posts),
        )
        .service(
            web::scope("/post")
                .service(handlers::post_handlers::one_post)
                .service(handlers::post_handlers::all_posts),
        );
}
