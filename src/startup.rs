use actix_web::middleware::Logger;
use actix_web::{
    dev::Server,
    web::{self},
    App, HttpServer,
};
use anyhow::Result;
use sqlx::PgPool;
use std::net::TcpListener;

use crate::routes::health_check::health_check;
use crate::routes::order::{buy, price, sell};
use crate::routes::user::{get_users, post_user};

pub fn run(listener: TcpListener, db_connection_pool: PgPool) -> Result<Server> {
    let db_connection = web::Data::new(db_connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/users", web::get().to(get_users))
            .route("/users", web::post().to(post_user))
            .route("/price/{ticker}", web::get().to(price))
            .route("/order/buy", web::post().to(buy))
            .route("/order/sell", web::post().to(sell))
            .app_data(db_connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
