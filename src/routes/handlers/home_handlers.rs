use crate::utils::{api_response::ApiResponse, app_status::AppState};
use actix_web::{get, web, Responder};
use sea_orm::{ConnectionTrait, DatabaseBackend, QueryResult, Statement};

#[get("")]
pub async fn home(app_state: web::Data<AppState>) -> Result<ApiResponse, ApiResponse> {
    let _response: Result<Vec<QueryResult>, ApiResponse> = app_state
        .db
        .query_all(Statement::from_string(
            DatabaseBackend::Postgres,
            "Select * from user;",
        ))
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()));

    Ok(ApiResponse::new(200, String::from("Welcome to Actix Web!")))
}

#[get("/hello/{name}")]
pub async fn hello(name: web::Path<String>) -> impl Responder {
    ApiResponse::new(200, format!("Hello {}!", name))
}
