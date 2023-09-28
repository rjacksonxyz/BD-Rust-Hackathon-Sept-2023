use crate::custom_error::Result;
use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};
use anyhow::anyhow;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{types::chrono, PgPool};
use uuid::Uuid;

use yahoo_finance_api::YahooConnector;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    user_id: String,
    ticker: String,
    quantity: i32,
    limit_order: bool,
    limit_price: f64,
}

/// Screw Actix and the no parameter logic.....
pub async fn buy(form: Json<Order>, db_pool: Data<PgPool>) -> Result<HttpResponse> {
    order(true, form, db_pool).await
}

/// Screw Actix and the no parameter logic.....
pub async fn sell(form: Json<Order>, db_pool: Data<PgPool>) -> Result<HttpResponse> {
    order(false, form, db_pool).await
}

async fn order(buy: bool, form: Json<Order>, db_pool: Data<PgPool>) -> Result<HttpResponse> {
    let ticket_price = price_ticker(form.ticker.to_string()).await?;

    match (form.limit_order, buy, ticket_price > form.limit_price) {
        (true, true, true) => {
            let s = format!(
                "Price is higher than limit_price by: {}",
                ticket_price - form.limit_price
            );
            log::warn!("{}", s);
            Ok(HttpResponse::BadRequest().body(s))
        }
        (true, false, false) => {
            let s = format!(
                "Price is lower than limit_price by: {}",
                form.limit_price - ticket_price
            );
            log::warn!("{}", s);
            Ok(HttpResponse::BadRequest().body(s))
        }
        _ => match add_transaction(buy, &*form, ticket_price, form.limit_order, db_pool).await {
            Ok(_) => Ok(HttpResponse::Ok().finish()),
            Err(e) => Ok(HttpResponse::BadRequest().body(format!("error: {}", e))),
        },
    }
}

pub async fn price(ticker: Path<String>) -> Result<HttpResponse> {
    match price_ticker(ticker.to_string()).await {
        Ok(price) => Ok(HttpResponse::Ok().json(price)),
        Err(e) => Ok(HttpResponse::BadRequest().body(format!(
            "Could retreive for ticker: {} with error: {}",
            ticker, e
        ))),
    }
}

async fn price_ticker(ticker: String) -> anyhow::Result<f64> {
    log::info!("Fetch price for ticker: {}", ticker.to_uppercase());

    match YahooConnector::new()
        .get_latest_quotes(&ticker.to_uppercase(), "1d")
        .await
    {
        Ok(resp) => Ok(resp.last_quote().unwrap().close),
        Err(e) => Err(anyhow!(
            "Could retreive for ticker: {} with error: {}",
            ticker,
            e
        )),
    }
}

async fn add_transaction(
    buy: bool,
    order: &Order,
    price: f64,
    order_type: bool,
    db_pool: Data<PgPool>,
) -> anyhow::Result<()> {
    log::info!("Adding transaction to db if valid");

    let order_type_string = match (buy, order_type) {
        (true, true) => "Limit Buy",
        (true, false) => "Market Buy",
        (false, true) => "Limit Sell",
        (false, false) => "Market Sell",
    };

    match sqlx::query(
        "INSERT INTO transaction_history
(id,user_id,asset_ticker,price,quantity,total_amount,purchased_at,order_type)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(Uuid::new_v4())
    .bind(&order.user_id)
    .bind(&order.ticker)
    .bind(price)
    .bind(order.quantity)
    .bind(order.quantity as f64 * price)
    .bind(Utc::now())
    .bind(order_type_string)
    .execute(&**db_pool)
    .await
    {
        Ok(_) => {
            log::info!("Transaction was succesfully added");
            Ok(())
        }
        Err(e) => {
            let s = format!("Transaction was NOT added with error: {}", e);
            log::error!("{}", s);
            Err(anyhow!("{}", s))
        }
    }
}
