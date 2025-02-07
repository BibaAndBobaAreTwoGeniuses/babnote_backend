use axum::{
    extract::State,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::{
    controllers::structs::ResponseAuthOk,
    crypt::{password, token},
    managers::{tokendb, userdb},
};

#[derive(Serialize, Deserialize)]
pub struct RegUser {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginUser {
    email: String,
    password: String,
}

pub async fn register_user(
    State(pool): State<MySqlPool>,
    Json(userdata): Json<RegUser>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if userdata.username.is_empty() || userdata.email.is_empty() || userdata.password.is_empty() {
        // Can't be too careful
        return Err((
            StatusCode::BAD_REQUEST,
            "One of the fields is empty".to_owned(),
        ));
    }
    let encrypted_password = password::encrypt_password(&userdata.password);
    if let Err(why) = userdb::create_user(
        &pool,
        &userdata.username,
        &userdata.email,
        &encrypted_password,
    )
    .await
    {
        println!("Error {}", why);
        return Err((
            StatusCode::BAD_REQUEST,
            "Could not register a user".to_owned(),
        ));
    }

    let user_id: i32 = userdb::get_user_id(&pool, &userdata.username)
        .await
        .unwrap(); // User id should be there like 100% percent (kinda)
    let token = token::make_token(user_id);

    tokendb::insert_token(&pool, user_id, &token)
        .await
        .expect("Token should've been inserted");

    Ok(Json(ResponseAuthOk::new(token)))
}

pub async fn login_user(
    State(pool): State<MySqlPool>,
    Json(userdata): Json<LoginUser>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    println!("here");
    if userdata.email.is_empty() || userdata.password.is_empty() {
        // Can't be too careful
        return Err((
            StatusCode::BAD_REQUEST,
            "One of the fields is empty".to_owned(),
        ));
    }

    let user_id: i32 = match userdb::get_user_id_by_email(&pool, &userdata.email).await {
        Ok(id) => id,
        Err(why) => {
            eprintln!("Error {}", why);
            return Err((StatusCode::BAD_REQUEST, "User does not exist".to_owned()));
        }
    };

    let hashed_password = userdb::get_password(&pool, user_id).await.unwrap(); // At this point user does exist
    if !password::validate_password(&userdata.password, &hashed_password) {
        return Err((StatusCode::UNAUTHORIZED, "Wrong credentials".to_owned()));
    }

    let token = token::make_token(user_id);
    println!("log succes");
    Ok(Json(ResponseAuthOk::new(token)))
}
