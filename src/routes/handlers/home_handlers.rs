use crate::utils::api_response;
use actix_web::{get, web, Responder};

#[get("/")]
pub async fn home() -> impl Responder {
    api_response::ApiResponse::new(200, String::from("Welcome to Actix Web!"))
}

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    api_response::ApiResponse::new(200, format!("Hello {}!", name))
}
