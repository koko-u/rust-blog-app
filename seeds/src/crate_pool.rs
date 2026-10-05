pub async fn create_pool() -> Result<sqlx::PgPool, sqlx::Error> {
    let database_url = env!("DATABASE_URL");
    sqlx::postgres::PgPoolOptions::new().connect(database_url).await
}
