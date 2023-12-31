#![feature(async_closure)]

use std::{net::SocketAddr, time::Duration};

use crate::localization_line::load::load_localization_lines_from_file;
use axum::routing::post;
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use tracing::log;

mod axum_common;
mod config;
pub mod localization_line;
mod parse;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let config = config::load_config(None);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // setup connection pool
    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .acquire_timeout(Duration::from_secs(3))
        .connect(config.connection_string.as_str())
        .await
        .expect("can't connect to database");

    // build our application with some routes
    let app = Router::new()
        .route(
            "/localization_line",
            post(localization_line::routes::create_localization_line)
                .get(localization_line::routes::get_localization_lines),
        )
        .with_state(pool.clone());

    load_localization_lines_from_file(&pool, config.localization_root).await;

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
