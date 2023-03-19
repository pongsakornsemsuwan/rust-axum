mod routes;
mod services;
mod models;
mod utils;
mod dao;
mod errors;

use axum::{Router, routing::{get}};
use log::LevelFilter;
use routes::promotion_route::*;
use sqlx::PgPool;
use utils::db_connection;
use simplelog::{TermLogger, TerminalMode, Config, ColorChoice};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

#[tokio::main]
async fn main() {
    
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Stdout,
        ColorChoice::Always
    ).unwrap();

    // 1. Getting connection pool and set to application state
    let pool = db_connection::get_connection_pool()
        .await
        .expect("Should be able to get connection pool");

    println!("connect pool successfully");

    
    let app_state = AppState { pool };

    // 2. Register routes
    let app = Router::new().route("/", get(|| async { "Hello, World!" }))
    .route("/promotions", get(get_all_promotions).post(add_promotion))
    .route("/promotion/:promotion_id", get(get_promotion).patch(update_promotion))
    // .route("/game-providers", get())
    .with_state(app_state);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}