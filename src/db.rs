use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
pub async fn init_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    SqlitePoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect(database_url)
        .await
}
