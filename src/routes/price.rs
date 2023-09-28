use crate::custom_error::Result;
use actix_web::{web::Path, HttpResponse};

use yahoo_finance_api::YahooConnector;

pub async fn price(ticker: Path<String>) -> Result<HttpResponse> {
    log::info!("Fetch price for ticker: {}", ticker.to_uppercase());

    match YahooConnector::new()
        .get_latest_quotes(&ticker.to_uppercase(), "1d")
        .await
    {
        Ok(resp) => Ok(HttpResponse::Ok().json(resp.last_quote().unwrap().close)),
        Err(e) => Ok(HttpResponse::BadRequest().body(format!(
            "Could retreive for ticker: {} with error: {}",
            ticker, e
        ))),
    }
}
