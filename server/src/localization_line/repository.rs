use sea_query::{PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{Acquire, Error, Postgres, Transaction};
use crate::localization_line::{domain, table};
use crate::localization_line::domain::TranslationVariant;
use crate::localization_line::usecase::CreateTranslationVariant;




pub(crate) async fn create(tx: &mut Transaction<'_, Postgres>, localization_line: CreateTranslationVariant) -> Result<domain::TranslationVariant, Error> {
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
            localization_line.file_path.clone().into(),
            localization_line.localization_date.into(),
            localization_line.lang.clone().into(),
            localization_line.version.into(),
            localization_line.text.clone().into(),
        ])
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);
    Ok(sqlx::query_as_with::<_, domain::TranslationVariant, _>(&query, values).fetch_one(tx.acquire().await?).await?)
}

pub(crate) async fn all(pool: &mut Transaction<'_, Postgres>, offset: Option<u64>, limit: Option<u64>) -> Result<Vec<TranslationVariant>, Error> {
    let mut tx = pool.begin().await?;

    let (query, values) = Query::select()
        .columns([
            table::TranslationVariant::Id,
            table::TranslationVariant::Text,
            table::TranslationVariant::Lang,
            table::TranslationVariant::Contentuid,
            table::TranslationVariant::Version,
            table::TranslationVariant::LocalizationDate,
            table::TranslationVariant::FilePath,
        ])
        .from(table::TranslationVariant::Table)
        .offset(offset.unwrap_or(0))
        .limit(limit.unwrap_or(i64::MAX as u64))
        .build_sqlx(PostgresQueryBuilder);
    Ok(sqlx::query_as_with::<_, domain::TranslationVariant, _>(&query, values).fetch_all(tx.acquire().await?).await?)
}