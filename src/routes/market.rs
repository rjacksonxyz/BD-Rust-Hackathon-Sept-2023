use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::{PgPool, Pool, Postgres};
use std::fmt;
use uuid::Uuid;
use yahoo::YahooError;
use yahoo_finance_api as yahoo;

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow, Debug)]
pub struct Order {
    user_id: String,
    ticker: String,
    quantity: u64,
    limit_order: bool,
    limit_price: f64,
}

struct OrderExecutable {
    order: Order,
    is_buy: bool,
    execution_price: f64,
    db_pool: web::Data<Pool<Postgres>>,
}

struct OrderError {
    message: String,
}

impl OrderError {
    pub fn new(message: &str) -> Self {
        OrderError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for OrderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub async fn get_price(ticker: web::Path<String>) -> HttpResponse {
    let ticker_str = ticker.into_inner();
    match last_quote(ticker_str.clone()).await {
        Ok(price) => HttpResponse::Ok().json(price),
        Err(e) => match e {
            YahooError::FetchFailed(_) => {
                let msg = format!("Unable to find price for ticker: {}", ticker_str);
                HttpResponse::NotFound().json(msg)
            }
            _ => HttpResponse::InternalServerError().json(e.to_string()),
        },
    }
}

pub async fn order_buy(order: web::Json<Order>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let order_executable = OrderExecutable {
        order: order.into_inner(),
        is_buy: true,
        execution_price: 0 as f64,
        db_pool,
    };
    let result = order_execute(order_executable).await;
    match result {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn order_sell(order: web::Json<Order>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let order_executable = OrderExecutable {
        order: order.into_inner(),
        is_buy: false,
        execution_price: 0 as f64,
        db_pool,
    };
    let result = order_execute(order_executable).await;
    match result {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

async fn order_execute(mut ox: OrderExecutable) -> Result<(), OrderError> {
    // get ticker (always needed)
    log::info!("Executing order...");
    match last_quote(ox.order.ticker.clone()).await {
        Ok(price) => {
            ox.execution_price = price;
        }
        Err(e) => match e {
            YahooError::FetchFailed(_) => {
                let msg = format!("Unable to find price for ticker: {}", ox.order.ticker);
                return Err(OrderError::new(&msg));
            }
            _ => return Err(OrderError::new((e.to_string().as_str()))),
        },
    };
    // determine if price is acceptable
    let valid_limit_order = ox.order.limit_order
        && limit_is_right_price(ox.is_buy, ox.order.limit_price, ox.execution_price);
    if valid_limit_order || !ox.order.limit_order {
        return submit_order_to_exchange(ox).await;
    }
    let price_delta = ox.order.limit_price - ox.execution_price;
    Err(OrderError::new(
        format!(
            "unable to execute order: {:?} | price delta ('-' for buy, implied '+' for sell): {}",
            ox.order, price_delta
        )
        .as_str(),
    ))
}

#[warn(clippy::collapsible_else_if)]
fn limit_is_right_price(is_buy: bool, limit_price: f64, market_price: f64) -> bool {
    if is_buy {
        limit_price >= market_price
    } else {
        limit_price <= market_price
    }
}

async fn submit_order_to_exchange(mut ox: OrderExecutable) -> Result<(), OrderError> {
    log::info!("Submitting order to exchange...",);
    let order_type: &str = match (ox.is_buy, ox.order.limit_order) {
        (true, true) => "Limit Buy",
        (true, false) => "Market Buy",
        (false, true) => "Limit Sell",
        (false, false) => "Market Sell",
    };
    let result = sqlx::query!(
        r#"
        INSERT INTO transaction_history (id, user_id, asset_ticker, price, quantity, total_amount, purchased_at, order_type)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
        Uuid::new_v4(),
        ox.order.user_id,
        ox.order.ticker,
        ox.execution_price,
        ox.order.quantity as i64,
        (ox.execution_price * ox.order.quantity as f64),
        Utc::now(),
        order_type
    )
    .execute(ox.db_pool.get_ref())
    .await;
    match result {
        Ok(_) => {
            log::info!("Order submitted to exchange!");
            Ok(())
        }
        Err(e) => {
            log::error!("Unable to submit order to exchange!");
            Err(OrderError::new(e.to_string().as_str()))
        }
    }
}

async fn last_quote(symbol: String) -> Result<f64, YahooError> {
    log::info!("Retrieving price quote for {}...", symbol.clone());
    let provider = yahoo::YahooConnector::new();
    let result = provider.get_latest_quotes(&symbol, "1d").await;
    let response: yahoo::YResponse = match result {
        Ok(yr) => yr,
        Err(e) => {
            return Err(e);
        }
    };
    match response.last_quote() {
        Ok(price) => Ok(price.close),
        Err(e) => Err(e),
    }
}
