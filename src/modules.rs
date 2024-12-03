use anyhow::Result;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Creator {
    pub id: i64,
    pub mint: String,
    pub creator: String,
    pub symbol: String,
    pub created_at: i64,
}

pub async fn establish_connection(database_file: &str) -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .connect(database_file)
        .await
        .expect(
            format!(
                "Failed to connect to database, please check the database file: {}",
                database_file
            )
            .as_str(),
        );

    pool
}

// judge if the record exists
pub async fn record_exists(pool: &SqlitePool, mint: &str) -> Result<bool> {
    let record = sqlx::query!("SELECT * FROM creator WHERE mint = ?", mint)
        .fetch_optional(pool)
        .await?;

    Ok(record.is_some())
}

pub async fn insert_record(
    pool: &SqlitePool,
    mint: &str,
    creator: &str,
    symbol: &str,
) -> Result<()> {
    let current_ts = chrono::Local::now().timestamp();
    sqlx::query!(
        "INSERT INTO creator (mint, creator, symbol, created_at) VALUES (?, ?, ?, ?)",
        mint,
        creator,
        symbol,
        current_ts
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_record_by_mint(pool: &SqlitePool, mint: &str) -> Result<Creator> {
    let record = sqlx::query_as("SELECT * FROM creator WHERE mint = ?")
        .bind(mint)
        .fetch_one(pool)
        .await?;

    Ok(record)
}

pub async fn get_records(pool: &SqlitePool) -> Result<Vec<Creator>> {
    let records = sqlx::query_as("SELECT * FROM creator")
        .fetch_all(pool)
        .await?;
    Ok(records)
}
