use axum::{extract::{DefaultBodyLimit, Multipart, Query}, http::{header::{CONTENT_DISPOSITION, CONTENT_TYPE}, HeaderMap, HeaderValue, StatusCode}, response::IntoResponse, routing::{get, post}, Router};

mod controllers;
mod managers;
mod crypt;

use controllers::{auth, note};
use serde::Deserialize;
use sqlx::{pool::PoolOptions, MySql};
use tokio::io::AsyncReadExt;
use std::{fs::{File, OpenOptions}, io::Write, time::Duration};


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let con_str = "mysql://klewy:root@localhost:3306/babnote";

    let pool = sqlx::mysql::MySqlPoolOptions::new()
    .max_connections(10)
    .acquire_timeout(Duration::from_secs(5))
    .connect(&con_str)
    .await
    .expect("Cant connect fuck it");

  
    println!("Connected");

    

    let app = Router::new()
    .route("/upload", post(note::upload))
    .route("/download", get(note::download))
    .route("/register", post(auth::register_user))
    .route("/login", post(auth::login_user))
    .layer(DefaultBodyLimit::max(100000))
    .with_state(pool);


    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
