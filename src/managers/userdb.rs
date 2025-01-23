use sqlx::MySqlPool;
use sqlx::Row;


pub async fn create_user(pool: &MySqlPool, username: &str, email: &str, password: &str) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO users (username, email, password) VALUES (?, ?, ?)")
    .bind(username)
    .bind(email)
    .bind(password)
    .execute(pool).await?;

    Ok(())
}

pub async fn get_user_id(pool: &MySqlPool, username: &str) -> anyhow::Result<i32> {
    let query: sqlx::mysql::MySqlRow = sqlx::query("SELECT id FROM users WHERE username = ?")
    .bind(username)
    .fetch_one(pool).await?;

    let id: i32 = query.try_get(0)?;
    Ok(id)
}

pub async fn get_user_id_by_email(pool: &MySqlPool, email: &str) -> anyhow::Result<i32> {
    let query: sqlx::mysql::MySqlRow = sqlx::query("SELECT id FROM users WHERE email = ?")
    .bind(email)
    .fetch_one(pool).await?;

    let id: i32 = query.try_get(0)?;
    Ok(id)
}

pub async fn get_username(pool: &MySqlPool, id: i32) -> anyhow::Result<String> {
    let query: sqlx::mysql::MySqlRow = sqlx::query("SELECT username FROM users WHERE id = ?")
    .bind(id)
    .fetch_one(pool).await?;

    let username: String = query.try_get(0)?;
    Ok(username)
}

pub async fn get_password(pool: &MySqlPool, id: i32) -> anyhow::Result<String> {
    let query: sqlx::mysql::MySqlRow = sqlx::query("SELECT password FROM users WHERE id = ?")
    .bind(id)
    .fetch_one(pool)
    .await?;

    let password: String = query.try_get(0)?;
    Ok(password)
}
