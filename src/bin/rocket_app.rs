use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{Config, State, get, main, post, routes};
use rust_web_trio::common::database::{
    ApplicationState, create_connection_pool, ensure_users_schema_exists,
};
use rust_web_trio::common::models::{NewUser, User};
use rust_web_trio::common::repository;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::LevelFilter::INFO)
        .with(fmt::layer().with_target(false).compact())
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("Should be able to get DATABASE_URL from environment got ");
    let database_connection_pool = create_connection_pool(&database_url, 10)
        .await
        .expect("Should be able to create connection pool got ");
    ensure_users_schema_exists(&database_connection_pool)
        .await
        .expect("Should be able to ensure users schema exists got ");

    let application_state = ApplicationState {
        database_connection_pool,
    };

    let figment = Config::figment().merge(("log_level", rocket::config::LogLevel::Normal));

    tracing::info!("Rocket taking off...");
    rocket::custom(figment)
        .manage(application_state)
        .mount("/", routes![health, get_user, post_user])
        .launch()
        .await
        .expect("Should have taking off got ");

    Ok(())
}

#[get("/health")]
fn health() -> &'static str {
    "Ok"
}

#[get("/users/<id>")]
async fn get_user(id: i64, state: &State<ApplicationState>) -> Result<Json<User>, Status> {
    match repository::get_user(&state.database_connection_pool, id).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/users", data = "<payload>")]
async fn post_user(
    payload: Json<NewUser>,
    state: &State<ApplicationState>,
) -> Result<(Status, Json<User>), Status> {
    repository::create_user(&state.database_connection_pool, payload.into_inner())
        .await
        .map(|u| (Status::Created, Json(u)))
        .map_err(|_| Status::InternalServerError)
}
