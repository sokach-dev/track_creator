use crate::config::Config;
use crate::modules::{insert_record, record_exists};
use reqwest;
use serde::Deserialize;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::time::Duration;
use tokio::time;
use tracing::info;

/*
CREATE TABLE creator (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    mint TEXT NOT NULL UNIQUE, -- mint address
    creator TEXT NOT NULL, -- creator address
    symbol TEXT NOT NULL, -- symbol
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
 */

#[derive(Deserialize)]
pub struct CoinData {
    pub mint: String,
    pub symbol: String,
    pub creator: String,
}

pub async fn start_scheduler(config: Config) {
    let pool = SqlitePoolOptions::new()
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    let interval = Duration::from_secs(config.interval_seconds);
    let mut ticker = time::interval(interval);

    let pump_url = config.pump_mc_url.clone();

    loop {
        ticker.tick().await;
        if let Err(e) =
            fetch_and_store_data(&pool, pump_url.as_str(), config.pump_mc_url_index).await
        {
            tracing::error!("Error fetching and storing data: {:?}", e);
        }
    }
}

async fn fetch_and_store_data(
    pool: &SqlitePool,
    pump_url: &str,
    max_index: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let limit = 50;
    for i in 0..max_index {
        let offset = i * 50;
        let pump_url = format!("{}&offset={}&limit={}", pump_url, offset, limit);
        info!("Fetching data from pump url: {}", pump_url);
        let resp = reqwest::get(pump_url)
            .await?
            .json::<Vec<CoinData>>()
            .await?;

        for coin in resp {
            let mint = coin.mint.clone();
            let creator = coin.creator.clone();
            let symbol = coin.symbol.clone();

            if record_exists(&pool, mint.as_str()).await? {
                info!("Record already exists for mint: {}", mint);
                continue;
            }

            insert_record(&pool, mint.as_str(), creator.as_str(), symbol.as_str()).await?;
        }

        info!("Data fetched and stored successfully");
    }
    Ok(())
}
