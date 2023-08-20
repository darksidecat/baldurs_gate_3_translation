use crate::localization_line::usecase::CreateTranslationVariant;
use crate::localization_line::usecase::{all, create};
use axum::Json;

use crate::axum_common::DatabaseConnection;
use crate::localization_line::domain::TranslationVariant;
use axum::extract::Query;
use axum::http::StatusCode;
use serde::Deserialize;

pub(crate) async fn create_localization_line(
    DatabaseConnection(mut conn): DatabaseConnection,
    Json(payload): Json<CreateTranslationVariant>,
) -> Result<Json<TranslationVariant>, (StatusCode, String)> {
    let result = create(&mut conn, payload).await.unwrap();
    Ok(Json(result))
}

#[derive(Default, Deserialize)]
pub struct LimitOffset {
    pub offset: Option<u64>,
    pub limit: Option<u64>,
}

pub(crate) async fn get_localization_lines(
    DatabaseConnection(mut conn): DatabaseConnection,
    offset_limit: Option<Query<LimitOffset>>,
) -> Result<Json<Vec<TranslationVariant>>, (StatusCode, String)> {
    let offset_limit = offset_limit.unwrap_or_default();
    let offset = offset_limit.offset;
    let limit = offset_limit.limit;

    let result = all(&mut conn, offset, limit).await.unwrap();
    Ok(Json(result))
}
