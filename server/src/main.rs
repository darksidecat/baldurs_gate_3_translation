use std::{net::SocketAddr, time::Duration};

use axum::Router;
use axum::routing::post;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod localization_line;
mod axum_common;
mod parse;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_connection_str = std::env::var("DATABASE_URL")
        .expect("can`t connect to database");

    // setup connection pool
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    // build our application with some routes
    let app = Router::new()
        .route("/localization_line", post(localization_line::routes::create_localization_line).get(localization_line::routes::get_localization_lines))
        .with_state(pool);

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    /*
    let file= File::open("./data/ukrainian.xml")?;
    let lines: Vec<LocalizationLine> = parse_translation(file)?;

    for i in lines.iter().take(10) {
        match repository::create(& mut pool, i).await {
            Ok(_) => {}
            Err(_) => {}
        }
    }*/
}

