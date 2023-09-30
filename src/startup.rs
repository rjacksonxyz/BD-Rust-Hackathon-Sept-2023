use crate::routes::{add_user, get_price, get_users, health_check, order_buy, order_sell};
use actix_web::middleware::Logger;
use actix_web::{
    dev::Server,
    web::{self},
    App, HttpServer,
};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_connection_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_connection = web::Data::new(db_connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/users", web::get().to(get_users))
            .route("/users", web::post().to(add_user))
            .route("/price/{ticker}", web::get().to(get_price))
            .route("/order/buy", web::post().to(order_buy))
            .route("/order/sell", web::post().to(order_sell))
            .app_data(db_connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
