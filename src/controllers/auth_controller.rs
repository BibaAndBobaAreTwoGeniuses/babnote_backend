use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::{crypt::password, managers::user_manager};

#[derive(Serialize, Deserialize)]
pub struct RegUser {
    username: String,
    email: String,
    password: String,
}



pub async fn register_user(State(pool): State<MySqlPool>, Json(userdata): Json<RegUser>) -> Result<impl IntoResponse, (StatusCode, String)> {
    if userdata.username.is_empty() && userdata.email.is_empty() && userdata.password.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "One of the fields is empty".to_owned()))
    }
    let encrypted_password = password::encrypt_password(&userdata.password);
    if let Err(why) = user_manager::create_user(&pool, &userdata.username, &userdata.email, &encrypted_password).await {
        println!("Error {}", why);
        return Err((StatusCode::BAD_REQUEST, "Could not register a user".to_owned()))
    }

    todo!("Make a token, insert it into database for tokens");
    
    Ok(())
}