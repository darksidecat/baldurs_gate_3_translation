use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub struct LocalizationLine {
    pub contentuid: String,
    pub version: i32,
    pub text: String,
}

#[derive(sqlx::FromRow, Debug)]
struct TranslationLocation {
    contentuid: Uuid,
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct TranslationVariant {
    id: Uuid,
    contentuid: String,
    file_path: String,
    localization_date: NaiveDateTime,
    lang: String,
    version: i32,
    text: String,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
}

#[derive(sqlx::FromRow, Debug)]
struct Vocabulary {
    id: Uuid,
    word: String,
}

#[derive(sqlx::FromRow, Debug)]
struct VocabularyUsages {
    id: Uuid,
    word_id: Uuid,
    translation_variant_id: Uuid,
}

//create in db
struct VocabularyTag {
    id: Uuid,
    tag: String,
}

//create in db
struct VocabularyTags {
    word_id: Uuid,
    tag_id: Uuid,
}
