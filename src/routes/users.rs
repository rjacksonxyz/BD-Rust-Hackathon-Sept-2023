use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct User {
    email: String,
    name: String,
    user_id: String,
}

pub async fn get_users(db_pool: web::Data<PgPool>) -> HttpResponse {
    let results: Result<Vec<User>, sqlx::Error> =
        sqlx::query_as("SELECT email, name, user_id FROM users")
            .fetch_all(&**db_pool)
            .await;

    match results {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            log::error!("Error retrieving user info from database: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn add_user(form: web::Json<User>, db_pool: web::Data<PgPool>) -> HttpResponse {
    log::info!(
        "Adding '{}' '{}' as a new user '{}'.",
        form.email,
        form.name,
        form.user_id
    );
    let result = sqlx::query!(
        r#"
        INSERT INTO users (id, email, name, user_id, subscribed_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        form.user_id,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .await;
    match result {
        Ok(_) => {
            log::info!("New user successfully added √",);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!("Failed to add user: {:?}", e);
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}
