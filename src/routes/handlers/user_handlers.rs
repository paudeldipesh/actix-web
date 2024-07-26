use crate::utils::{api_response::ApiResponse, app_status::AppState, jwt::Claims};
use actix_web::{get, post, web};
use entity::user::{ActiveModel, Entity};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct UpdateUserModel {
    name: String,
}

#[get("")]
pub async fn user(
    app_state: web::Data<AppState>,
    claim_data: Claims,
) -> Result<ApiResponse, ApiResponse> {
    let user_model = entity::user::Entity::find_by_id(claim_data.id)
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .ok_or(ApiResponse::new(404, "user not found".to_owned()))?;

    let entity::user::Model { name, email, .. } = user_model;

    Ok(ApiResponse::new(
        200,
        format!("{{ 'name': '{}', 'email': '{}' }}", name, email),
    ))
}

#[post("update")]
pub async fn update_user(
    app_state: web::Data<AppState>,
    user_data: web::Json<UpdateUserModel>,
    claim_data: Claims,
) -> Result<ApiResponse, ApiResponse> {
    let mut user_model: ActiveModel = Entity::find_by_id(claim_data.id)
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .ok_or(ApiResponse::new(404, "user not found".to_owned()))?
        .into_active_model();

    user_model.name = Set(user_data.name.clone());
    user_model
        .update(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, "user updated".to_owned()))
}
