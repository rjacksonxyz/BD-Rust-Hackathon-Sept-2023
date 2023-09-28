use actix_web::{web, HttpResponse, Result};
use serde::Serialize;
use sqlx::PgPool;
use sqlx::Row;

#[derive(sqlx::FromRow, Serialize)]
struct User {
    email: String,
    name: String,
    user_id: String,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    message: String,
}

pub async fn get_users(db_pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let result = sqlx::query("SELECT email, name, user_id FROM users")
        .fetch_all(db_pool.get_ref())
        .await;

    match result {
        Ok(rows) => {
            let users: Vec<User> = rows
                .into_iter()
                .map(|row| User {
                    email: row.get("email"),
                    name: row.get("name"),
                    user_id: row.get("user_id"),
                })
                .collect();

            Ok(HttpResponse::Ok().json(users))
        }
        Err(err) => {
            log::error!("Error querying database: {:?}", err);
            let error_response = ErrorResponse {
                message: "Internal Server Error".to_string(),
            };
            Ok(HttpResponse::InternalServerError().json(error_response))
        }
    }
}
