use sqlx::{Pool, Postgres};

pub type AsyncPostgresConnectionPool = Pool<Postgres>;

pub async fn create_connection_pool(
    database_url: &str,
    max_number_of_connections: u32,
) -> anyhow::Result<AsyncPostgresConnectionPool> {
    let connection_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(max_number_of_connections)
        .connect(database_url)
        .await?;
    Ok(connection_pool)
}

pub async fn ensure_users_schema_exists(
    connection_pool: &AsyncPostgresConnectionPool,
) -> anyhow::Result<()> {
    // TODO Change to sqlx migrate for production grade code
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users(
            id BIGINT PRIMARY KEY,
            name TEXT NOT NULL
        );
    "#,
    )
    .execute(connection_pool)
    .await?;
    Ok(())
}
