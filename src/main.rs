mod routes;
mod services;
mod models;
// mod utils;

use axum::{Router, routing::get};
use routes::promotion_route::*;


#[tokio::main]
async fn main() {
    
    println!("connect pool successfully");

    let app = Router::new().route("/", get(|| async { "Hello, World!" }))
    .route("/promotions", get(get_all_promotions))
    .route("/promotion/:promotion_id", get(get_promotion));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}