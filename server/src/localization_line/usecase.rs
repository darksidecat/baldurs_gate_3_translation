use sqlx::pool::PoolConnection;
use sqlx::{Error, Postgres};
use sqlx::Acquire;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};
use crate::localization_line::{domain, repository};

pub async fn create(pool: &mut PoolConnection<Postgres>, localization_line: CreateTranslationVariant) -> Result<domain::TranslationVariant, Error> {
    let mut tx = pool.begin().await.unwrap();

    let result = repository::create(&mut tx, localization_line).await;
    tx.commit().await?;
    result
}

pub async fn all(pool: &mut PoolConnection<Postgres>, offset: Option<u64>, limit: Option<u64>) -> Result<Vec<domain::TranslationVariant>, Error> {
    let mut tx = pool.begin().await.unwrap();

    let result = repository::all(&mut tx, offset, limit).await?;
    tx.commit().await?;
    Ok(result)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTranslationVariant{
    pub contentuid: String,
    pub file_path: String,
    pub localization_date: NaiveDateTime,
    pub lang: String,
    pub version: i32,
    pub text: String,
}
