use std::{fs::OpenOptions, io::Write};

use axum::{extract::{Multipart, Query, State}, http::{header::{CONTENT_DISPOSITION, CONTENT_TYPE}, HeaderMap, HeaderValue, StatusCode}, response::IntoResponse};
use sqlx::MySqlPool;
use tokio::io::AsyncReadExt;

use crate::{crypt::token, managers::tokendb};

use super::structs::DownloadReq;



// /upload; headers: Authorization; Body: multipart
pub async fn upload(State(pool): State<MySqlPool>, headers: HeaderMap, mut multipart: Multipart) -> Result<impl IntoResponse, (StatusCode, String)> {
    let empty = HeaderValue::from_str("").unwrap();
    let token = headers.get("Authorization")
    .unwrap_or(&empty)
    .to_str()
    .unwrap()
    .split_whitespace()
    .nth(1).expect("Token is not set");


    let user_id = match token::verify_token(token) {
        Ok(id) => id,
        Err(why) => {
            eprintln!("Error {}", why);
            return Err((StatusCode::UNAUTHORIZED, "Token was not verified".into()))
        }
    };
    if !tokendb::token_exists(&pool, &token).await {
        return Err((StatusCode::UNAUTHORIZED, "Token was not verified".into()))
    }

    while let Some(field) = multipart.next_field().await.unwrap() {
        let dir_name = user_id.to_string();
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

// /upload; headers: Authorization; Body: filename String
pub async fn download(State(pool): State<MySqlPool>, headers: HeaderMap, Query(download_req) : Query<DownloadReq>) 
    -> Result<impl IntoResponse, (StatusCode, String)> {

    let empty = HeaderValue::from_str("").unwrap();
    let token = headers.get("Authorization")
    .unwrap_or(&empty)
    .to_str()
    .unwrap()
    .split_whitespace()
    .nth(1).expect("Token is not set");


    let user_id = match token::verify_token(token) {
        Ok(id) => id,
        Err(why) => {
            eprintln!("Error {}", why);
            return Err((StatusCode::UNAUTHORIZED, "Token was not verified".into()))
        }
    };
    if !tokendb::token_exists(&pool, &token).await {
        return Err((StatusCode::UNAUTHORIZED, "Token was not verified".into()))
    }

    let path = user_id.to_string() + "/" + download_req.filename.as_str();
    let mut file = match tokio::fs::File::open(path).await {
        Ok(file) => file,
        Err(why) => {
            return Err((StatusCode::NOT_FOUND, why.to_string()))
        }
    };
    
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await.expect("Failed to read");

    let mut headers = HeaderMap::new();
    headers.append(CONTENT_TYPE, HeaderValue::from_str("application/vnd.sqlite3").unwrap());
    headers.append(CONTENT_DISPOSITION, HeaderValue::from_str(format!("form-data; name=\"user\"; filename=\"babnote.sqlite\"").as_str()).unwrap());

    Ok((headers, buf))
}

