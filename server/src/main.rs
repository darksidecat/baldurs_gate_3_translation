#![feature(slice_first_last_chunk)]

mod parse;
mod domain;
mod table;
mod repository;
use std::env;

use std::fs::File;
use dotenv::dotenv;
use crate::parse::parse_translation;
use std::sync::Arc;
use tokio::sync::Mutex;
use sqlx::{Acquire, Error, Pool, Postgres};
use sqlx::postgres::any::AnyConnectionBackend;
use sqlx::postgres::PgPoolOptions;
use crate::domain::{LocalizationLine, TranslationVariant};

const MAX_CONNECTIONS: u32 = 20;

pub async fn establish_connection(database_url: &str) -> Pool<Postgres> {
    PgPoolOptions::new().max_connections(MAX_CONNECTIONS).connect(database_url).await.unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let connection = establish_connection(&database_url).await;
    let mut pool = connection.try_acquire().unwrap();

    // let a_poll = Arc::new(Mutex::new(pool));

    let file= File::open("./data/ukrainian.xml")?;
    let lines: Vec<LocalizationLine> = parse_translation(file)?;

    for i in lines.iter().take(10) {
        match repository::create(& mut pool, i).await {
            Ok(_) => {}
            Err(_) => {}
        }
    }

    Ok(())
}