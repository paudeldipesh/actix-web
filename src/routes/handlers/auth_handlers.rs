use crate::utils::{api_response::ApiResponse, app_status::AppState, jwt};
use actix_web::{get, post, web, Responder};
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, DbErr, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use sha256::digest;

#[derive(Serialize, Deserialize)]
struct RegisterModel {
    name: String,
    email: String,
    password: String,
}

#[post("/register")]
pub async fn register(
    app_state: web::Data<AppState>,
    register_json: web::Json<RegisterModel>,
) -> Result<ApiResponse, ApiResponse> {
    let user_model = entity::user::ActiveModel {
        name: Set(register_json.name.clone()),
        email: Set(register_json.email.clone()),
        password: Set(digest(&register_json.password)),
        ..Default::default()
    }
    .insert(&app_state.db)
    .await
    .map_err(|err: DbErr| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, format!("{}", user_model.id)))
}

#[derive(Serialize, Deserialize)]
struct LoginModel {
    email: String,
    password: String,
}

#[post("/login")]
pub async fn login(
    app_state: web::Data<AppState>,
    login_json: web::Json<LoginModel>,
) -> Result<ApiResponse, ApiResponse> {
    let user_data = entity::user::Entity::find()
        .filter(
            Condition::all()
                .add(entity::user::Column::Email.eq(&login_json.email))
                .add(entity::user::Column::Password.eq(digest(&login_json.password))),
        )
        .one(&app_state.db)
        .await
        .map_err(|err: DbErr| ApiResponse::new(500, err.to_string()))?
        .ok_or(ApiResponse::new(404, "User not found".to_owned()))?;

    let token: String = jwt::encode_jwt(user_data.email, user_data.id)
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, format!("{{ 'token': '{}' }}", token)))
}

#[get("/hi/{name}")]
pub async fn hi(name: web::Path<String>) -> impl Responder {
    ApiResponse::new(200, format!("Hi {}!", name))
}
