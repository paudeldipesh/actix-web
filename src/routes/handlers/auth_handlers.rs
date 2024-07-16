use crate::utils::{api_response, app_status::AppState, jwt};
use actix_web::{get, post, web, Responder};
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set};
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
) -> impl Responder {
    let user_model = entity::user::ActiveModel {
        name: Set(register_json.name.clone()),
        email: Set(register_json.email.clone()),
        password: Set(digest(&register_json.password)),
        ..Default::default()
    }
    .insert(&app_state.db)
    .await
    .unwrap();

    api_response::ApiResponse::new(200, format!("{}", user_model.id))
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
) -> impl Responder {
    let user = entity::user::Entity::find()
        .filter(
            Condition::all()
                .add(entity::user::Column::Email.eq(&login_json.email))
                .add(entity::user::Column::Password.eq(digest(&login_json.password))),
        )
        .one(&app_state.db)
        .await
        .unwrap();

    if user.is_none() {
        return api_response::ApiResponse::new(401, String::from("user not found"));
    }

    let user_data = user.unwrap();
    let token = jwt::encode_jwt(user_data.email, user_data.id).unwrap();
    api_response::ApiResponse::new(200, format!("{{ 'token': '{}' }}", token))
}

#[get("/hi/{name}")]
pub async fn hi(name: web::Path<String>) -> impl Responder {
    api_response::ApiResponse::new(200, format!("Hi {}!", name))
}
