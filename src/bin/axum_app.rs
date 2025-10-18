use axum::extract::{Path, State};
use axum::handler::Handler;
use axum::http::{Request, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use rust_web_trio::common::database::{
    create_connection_pool, ensure_users_schema_exists, AsyncPostgresConnectionPool,
};
use rust_web_trio::common::models::{NewUser, User};
use rust_web_trio::common::repository;
use rust_web_trio::common::repository::create_user;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber::fmt::writer::{MakeWriterExt, WithMaxLevel};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

#[derive(Clone)]
struct ApplicationState {
    database_connection_pool: AsyncPostgresConnectionPool,
}

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let database_url = std::env::var("DATABASE_URL")?;
    let database_connection_pool = create_connection_pool(&database_url, 10).await?;
    ensure_users_schema_exists(&database_connection_pool).await?;

    let application_router = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/users/{id}", get(get_user))
        .route("/users", post(post_user))
        .with_state(ApplicationState {
            database_connection_pool,
        })
        .layer(TraceLayer::new_for_http());

    let socket_address = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Axum listening on {socket_address}");
    let tcp_listener = TcpListener::bind(socket_address).await?;
    axum::serve(tcp_listener, application_router).await?;
    Ok(())
}

async fn get_user(
    State(state): State<ApplicationState>,
    Path(id): Path<i64>,
) -> Result<Json<User>, StatusCode> {
    match repository::get_user(&state.database_connection_pool, id).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn post_user(
    State(state): State<ApplicationState>,
    Json(payload): Json<NewUser>,
) -> Result<StatusCode, StatusCode> {
    create_user(&state.database_connection_pool, payload)
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
