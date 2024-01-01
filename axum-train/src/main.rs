use axum::{
    extract::Extension,
    routing::{delete, get, post},
    Router,Json
};
use std::net::SocketAddr;
// use std::{env, sync::Arc};
// use dotenv::dotenv;
// use hyper::header::CONTENT_TYPE;
// use sqlx::{PgPool, FromRow};
//use tower_http::cors::{Any, CorsLayer, Origin};

#[tokio::main]
async fn main() {
    //dotenv().ok();
    let app = Router::new().route("/", get(root));
    // 最近のaxumが少し変わった模様
    // let addr = SocketAddr::from(([0,0,0,1], 3000));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "200 ok"
}