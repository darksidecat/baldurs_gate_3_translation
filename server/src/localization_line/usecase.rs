use crate::localization_line::{domain, repository};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::pool::PoolConnection;
use sqlx::{Acquire, Executor};
use sqlx::{Error, Postgres};

pub async fn create(
    pool: &mut PoolConnection<Postgres>,
    localization_line: CreateTranslationVariant,
) -> Result<domain::TranslationVariant, Error> {
    let mut tx = pool.begin().await.unwrap();

    let result = repository::create(&mut tx, localization_line).await;
    tx.commit().await?;
    result
}

pub async fn create_many(
    mut pool: &mut PoolConnection<Postgres>,
    localization_line: &[CreateTranslationVariant],
) -> () {
    let mut tx = pool.begin().await.unwrap();

    let result = repository::create_many(&mut tx, localization_line).await;
    tx.commit().await.unwrap();
    println!("{:?}", result);
}

pub async fn all(
    pool: &mut PoolConnection<Postgres>,
    offset: Option<u64>,
    limit: Option<u64>,
) -> Result<Vec<domain::TranslationVariant>, Error> {
    let mut tx = pool.begin().await.unwrap();

    let result = repository::all(&mut tx, offset, limit).await?;
    tx.commit().await?;
    Ok(result)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTranslationVariant {
    pub contentuid: String,
    pub file_path: String,
    pub localization_date: NaiveDateTime,
    pub lang: String,
    pub version: i32,
    pub text: String,
}
