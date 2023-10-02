use crate::custom_error::Result;
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{types::chrono, PgPool};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct User {
    name: String,
    email: String,
    user_id: String,
}

pub async fn get_users(db_pool: Data<PgPool>) -> Result<HttpResponse> {
    log::info!("Pulling all users from DB");

    let users: Vec<User> = sqlx::query_as("SELECT * FROM users")
        .fetch_all(&**db_pool)
        .await?;

    log::info!("{:?}", users);

    Ok(HttpResponse::Ok().json(users))
}

pub async fn post_user(user: Json<User>, db_pool: Data<PgPool>) -> Result<HttpResponse> {
    log::info!("Adding user to db if valid");

    match sqlx::query(
        "INSERT INTO users (id,email,name,user_id,subscribed_at) VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(Uuid::new_v4())
    .bind(user.email.as_str())
    .bind(user.name.as_str())
    .bind(user.user_id.as_str())
    .bind(Utc::now())
    .execute(&**db_pool)
    .await
    {
        Ok(_) => {
            log::info!("User was succesfully added");
            Ok(HttpResponse::Ok().finish())
        }
        Err(e) => {
            log::error!("User was NOT added with error: {}", e);
            Ok(HttpResponse::BadRequest().body(format!(
                "user: {} was not added with error: {}",
                user.user_id, e
            )))
        }
    }
}
