use sqlx::MySqlPool;
use sqlx::Row;



pub async fn insert_token(pool: &MySqlPool, user_id: i32, token: &str) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO tokens (id, token) VALUES (?, ?)")
    .bind(user_id)
    .bind(token)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn token_exists(pool: &MySqlPool, token: &str) -> bool {
    let resp = sqlx::query("SELECT EXISTS(SELECT 1 FROM tokens WHERE token = ?)")
    .bind(token)
    .fetch_one(pool)
    .await.expect("Should have returned something"); // Returns 1 or 0

    resp.get::<bool, _>(0) 
}