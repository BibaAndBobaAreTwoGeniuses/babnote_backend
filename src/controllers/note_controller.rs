use std::{fs::OpenOptions, io::Write};

use axum::{extract::{Multipart, Query}, http::{header::{CONTENT_DISPOSITION, CONTENT_TYPE}, HeaderMap, HeaderValue, StatusCode}, response::IntoResponse};
use tokio::io::AsyncReadExt;

use crate::User;




pub async fn upload(mut multipart: Multipart) -> Result<impl IntoResponse, (StatusCode, String)> {
    println!("here");
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let dir_name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let data: axum::body::Bytes = field.bytes().await.unwrap();

        match upload_file(&dir_name, &file_name, data) {
            Ok(_) => {
                println!("success");
            },
            Err(why) => {
                eprintln!("gademn {}", why);
                return Err((StatusCode::BAD_REQUEST, why.to_string()))
            }
        }
    }
    Ok(())
}

#[axum::debug_handler]
pub async fn download(Query(username) : Query<User>) -> Result<impl IntoResponse, (StatusCode, String)> {
    println!("{}", username.username.clone() + "/babnote.sqlite");
    let mut file = match tokio::fs::File::open(username.username.clone() + "/babnote.sqlite").await {
        Ok(file) => file,
        Err(why) => {
            return Err((StatusCode::NOT_FOUND, why.to_string()))
        }
    };
    
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await.unwrap();

    let mut headers = HeaderMap::new();
    headers.append(CONTENT_TYPE, HeaderValue::from_str("application/vnd.sqlite3").unwrap());
    headers.append(CONTENT_DISPOSITION, HeaderValue::from_str(format!("form-data; name=\"{}\"; filename=\"babnote.sqlite\"", username.username).as_str()).unwrap());

    Ok((headers, buf))
}

fn upload_file(dir_name: &str, file_name: &str, data: axum::body::Bytes) -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new(dir_name);
    if !path.exists() {
        std::fs::create_dir(dir_name)?
    }
    let full_path = dir_name.to_string() + "/" + file_name;
    let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(full_path)?;
    file.write(&data).unwrap();
    Ok(())
}