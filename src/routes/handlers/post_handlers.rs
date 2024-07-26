use crate::utils::{api_response::ApiResponse, app_status::AppState, jwt::Claims};
use actix_web::{get, post, web};
use chrono::{NaiveDateTime, Utc};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct CreatePostModel {
    title: String,
    text: String,
}

#[derive(Serialize, Deserialize)]
struct UserModel {
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
struct PostModel {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub uuid: Uuid,
    pub image: Option<String>,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub user: Option<UserModel>,
}

// Create a post
#[post("create")]
pub async fn create_post(
    app_state: web::Data<AppState>,
    claim: Claims,
    post_model: web::Json<CreatePostModel>,
) -> Result<ApiResponse, ApiResponse> {
    let post_entity = entity::post::ActiveModel {
        title: Set(post_model.title.clone()),
        text: Set(post_model.text.clone()),
        uuid: Set(Uuid::new_v4()),
        user_id: Set(claim.id),
        created_at: Set(Utc::now().naive_local()),
        ..Default::default()
    };

    post_entity
        .insert(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, "Post Created".to_owned()))
}

// Get my posts
#[get("my-posts")]
pub async fn my_posts(
    app_state: web::Data<AppState>,
    claim: Claims,
) -> Result<ApiResponse, ApiResponse> {
    let posts: Vec<PostModel> = entity::post::Entity::find()
        .filter(entity::post::Column::UserId.eq(claim.id))
        .all(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .into_iter()
        .map(|post| PostModel {
            id: post.id,
            title: post.title,
            text: post.text,
            uuid: post.uuid,
            image: post.image,
            user_id: post.user_id,
            created_at: post.created_at,
            user: None,
        })
        .collect();
    let res_str =
        serde_json::to_string(&posts).map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, res_str.to_owned()))
}
// Get all posts
#[get("all-posts")]
pub async fn all_posts(app_state: web::Data<AppState>) -> Result<ApiResponse, ApiResponse> {
    let posts: Vec<PostModel> = entity::post::Entity::find()
        .all(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .into_iter()
        .map(|post: entity::post::Model| PostModel {
            id: post.id,
            title: post.title,
            text: post.text,
            uuid: post.uuid,
            image: post.image,
            user_id: post.user_id,
            created_at: post.created_at,
            user: None,
        })
        .collect();

    let res_str: String = serde_json::to_string(&posts)
        .map_err(|err: serde_json::Error| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, res_str.to_owned()))
}

// Get a single post
#[get("post/{post_uuid}")]
pub async fn one_post(
    app_state: web::Data<AppState>,
    post_uuid: web::Path<Uuid>,
) -> Result<ApiResponse, ApiResponse> {
    let posts: PostModel = entity::post::Entity::find()
        .filter(entity::post::Column::Uuid.eq(post_uuid.clone()))
        .find_also_related(entity::user::Entity)
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .map(|post| PostModel {
            id: post.0.id,
            title: post.0.title,
            text: post.0.text,
            uuid: post.0.uuid,
            image: post.0.image,
            user_id: post.0.user_id,
            created_at: post.0.created_at,
            user: post.1.map(|item| UserModel {
                name: item.name,
                email: item.email,
            }),
        })
        .ok_or(ApiResponse::new(404, "No Post Found".to_string()))?;

    let res_str =
        serde_json::to_string(&posts).map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, res_str.to_owned()))
}
