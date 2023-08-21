use crate::config::LocalizationRoot;
use crate::localization_line;
use crate::localization_line::domain::LocalizationLine;
use crate::localization_line::usecase::CreateTranslationVariant;
use crate::parse::parse_translation;
use chrono::NaiveDateTime;
use sqlx::{Pool, Postgres};
use std::fs::File;

pub async fn load_localization_lines_from_file(
    mut pool: &Pool<Postgres>,
    file_root: LocalizationRoot,
) -> () {
    let file = File::open(file_root.as_str().to_string() + "data/ukrainian.xml").unwrap();
    let lines: Vec<LocalizationLine> = parse_translation(file).unwrap();

    let mut data = vec![];

    for i in lines {
        data.push(CreateTranslationVariant {
            contentuid: i.contentuid.clone(),
            file_path: "ukrainian.xml".to_string(),
            localization_date: NaiveDateTime::parse_from_str(
                "2023-08-19 00:00:00",
                "%Y-%m-%d %H:%M:%S",
            )
            .unwrap(),
            lang: "uk".to_string(),
            version: i.version,
            text: i.text.clone(),
        })
    }

    let mut conn = pool.clone().acquire().await.unwrap();

    tokio::spawn(async move {
        for batch in data.chunks(2000) {
            localization_line::usecase::create_many(&mut conn, batch).await;
        }
    });
    ()
}
