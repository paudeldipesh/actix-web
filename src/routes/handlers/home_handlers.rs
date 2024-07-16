use crate::utils::{api_response, app_status::AppState};
use actix_web::{get, web, Responder};
use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};

#[get("")]
pub async fn home(app_state: web::Data<AppState>) -> impl Responder {
    let _response = app_state
        .db
        .query_all(Statement::from_string(
            DatabaseBackend::Postgres,
            "Select * from user;",
        ))
        .await
        .unwrap();
    api_response::ApiResponse::new(200, String::from("Welcome to Actix Web!"))
}

#[get("/hello/{name}")]
pub async fn hello(name: web::Path<String>) -> impl Responder {
    api_response::ApiResponse::new(200, format!("Hello {}!", name))
}
