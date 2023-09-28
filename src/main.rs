use anyhow::Result;
use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;
use tard_fi_data::startup::run;

const DATABASE_URL: &str = "postgres://postgres:password@127.0.0.1:5432/tard_fi_data";
const PORT: &str = "8080";

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let db_connection_pool = PgPool::connect(DATABASE_URL)
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", PORT);
    let listener = TcpListener::bind(address)?;
    Ok(run(listener, db_connection_pool)?.await?)
}
