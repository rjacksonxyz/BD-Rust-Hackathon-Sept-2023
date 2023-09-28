use actix_web::{error::Error, web, HttpResponse, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateUserIntent {
    pub name: String,
    pub email: String,
    pub user_id: String,
}

#[derive(Serialize)]
struct User {
    id: Uuid,
    email: String,
    name: String,
    user_id: String,
    subscribed_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    message: String,
}

pub async fn create_user(
    db_pool: web::Data<PgPool>,
    request: web::Json<CreateUserIntent>,
) -> Result<HttpResponse, Error> {
    let result = sqlx::query_as!(
        User,
        "INSERT INTO users (id, email, name, user_id, subscribed_at) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        Uuid::new_v4(), &request.email, &request.name, &request.user_id, Utc::now()
    )
    .fetch_one(db_pool.get_ref())
    .await;

    match result {
        Ok(user) => Ok(HttpResponse::Created().json(user)),
        Err(err) => {
            log::error!("Error inserting user into the database: {:?}", err);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                message: "Internal Server Error".to_string(),
            }))
        }
    }
}
