use crate::common::database::AsyncPostgresConnectionPool;
use crate::common::models::{NewUser, User};

pub async fn get_user(
    connection_pool: &AsyncPostgresConnectionPool,
    user_id: i64,
) -> sqlx::Result<Option<User>> {
    sqlx::query_as::<_, User>("SELECT id, name FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(connection_pool)
        .await
}

pub async fn create_user(
    connection_pool: &AsyncPostgresConnectionPool,
    u: NewUser,
) -> sqlx::Result<User> {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (id, name) VALUES ($1, $2)
         RETURNING id, name",
    )
    .bind(u.id)
    .bind(u.name)
    .fetch_one(connection_pool)
    .await
}
