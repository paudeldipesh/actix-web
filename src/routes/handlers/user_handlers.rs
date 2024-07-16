use crate::utils::{api_response, app_status::AppState};
use actix_web::{get, web, Responder};

#[get("")]
pub async fn user(_app_state: web::Data<AppState>) -> impl Responder {
    api_response::ApiResponse::new(200, String::from("verified user"))
}
