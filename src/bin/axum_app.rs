use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use rust_web_trio::common::database::{
    create_connection_pool, ensure_users_schema_exists, ApplicationState
};
use rust_web_trio::common::models::{NewUser, User};
use rust_web_trio::common::repository;
use rust_web_trio::common::repository::create_user;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::Level;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let database_url = std::env::var("DATABASE_URL").expect("Should be able to get DATABASE_URL from environment got ");
    let database_connection_pool = create_connection_pool(&database_url, 10).await.expect("Should be able to create connection pool got ");
    ensure_users_schema_exists(&database_connection_pool).await.expect("Should be able to ensure users schema exists got ");

    let application_router = Router::new()
        .route("/health", get(|| async { "Ok" }))
        .route("/users/{id}", get(get_user))
        .route("/users", post(post_user))
        .with_state(ApplicationState {
            database_connection_pool,
        })
        .layer(TraceLayer::new_for_http());

    let socket_address = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Axum listening on {socket_address}");
    let tcp_listener = TcpListener::bind(socket_address).await.expect("Should be able to bind tcp listener got ");
    axum::serve(tcp_listener, application_router).await.expect("Should be able to serve HTTP server got ");
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
