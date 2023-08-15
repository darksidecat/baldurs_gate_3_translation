use std::result;
use async_trait::async_trait;

use sea_query::{PostgresQueryBuilder, Query};

use sea_query_binder::SqlxBinder;
use sqlx::{Acquire, Connection, Error, PgConnection, PgPool, Postgres, Transaction};
use sqlx::pool::PoolConnection;
use sqlx::postgres::any::AnyConnectionBackend;

use sqlx::types::chrono::{Utc};
use crate::{domain, table};
use crate::domain::TranslationVariant;


pub(crate) async fn create(pool: &mut PoolConnection<Postgres>, localization_line: &domain::LocalizationLine) -> Result<domain::TranslationVariant, Error> {
    let mut tx = pool.begin().await.unwrap();
    let (query, values) = Query::insert()
        .into_table(table::TranslationLocation::Table)
        .columns([
            table::TranslationVariant::Contentuid,
        ])
        .values_panic([
            localization_line.contentuid.clone().into(),
        ])
        .build_sqlx(PostgresQueryBuilder);

    let result = sqlx::query_with(&query, values).execute(tx.acquire().await?).await;
    match result {
        Ok(_) => {}
        Err(err) => {return Err(err);}
    }

    let (query, values) = Query::insert()
        .into_table(table::TranslationVariant::Table)
        .columns([
            table::TranslationVariant::Contentuid,
            table::TranslationVariant::FilePath,
            table::TranslationVariant::LocalizationDate,
            table::TranslationVariant::Lang,
            table::TranslationVariant::Version,
            table::TranslationVariant::Text,
        ])
        .values_panic([
            localization_line.contentuid.clone().into(),
            "path".into(),
             Utc::now().into(),
            "UA".into(),
            localization_line.version.into(),
            localization_line.text.clone().into(),
        ])
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);


    let result = sqlx::query_as_with::<_, domain::TranslationVariant, _>(&query, values).fetch_one(tx.acquire().await?).await;
    match result {
        Ok(_) => {
            tx.commit().await?;
            result
        }
        Err(err) => {
            tx.rollback().await?;
            Err(err)
        }
    }
}


/*
pub struct WarehousePostgresReader {
    conn: Arc<Mutex<PoolConnection<Postgres>>>
}

impl WarehousePostgresReader {
    pub(crate) fn new(conn: Arc<Mutex<PoolConnection<Postgres>>>) -> Self {
        Self {conn}
    }
}

#[async_trait]
impl repository::WarehouseReader for WarehousePostgresReader {
    async fn all(&mut self) -> Vec<crate::warehouse::domain::Warehouse> {
        let (query, values) = Query::select()
            .columns([Warehouse::Id, Warehouse::Name])
            .from(Warehouse::Table)
            .limit(20)
            .build_sqlx(PostgresQueryBuilder);
        let mut conn = self.conn.lock().await;
        sqlx::query_as_with::<_, models::Warehouse, _>(&query, values).fetch_all(&mut *conn).await.unwrap()
    }
}
*/