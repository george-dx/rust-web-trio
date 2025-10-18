use actix_web::middleware::Logger;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use rust_web_trio::common::database::{
    ApplicationState, create_connection_pool, ensure_users_schema_exists,
};
use rust_web_trio::common::models::NewUser;
use rust_web_trio::common::repository;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[actix_web::main]
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

    let listening_address_tuple = ("127.0.0.1", 3001);
    tracing::info!(
        "Actix listening on {}:{}",
        listening_address_tuple.0,
        listening_address_tuple.1
    );

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(application_state.clone()))
            .service(health)
            .service(get_user)
            .service(post_user)
    })
    .bind(listening_address_tuple)
    .expect("Should be able to bind listening address got ")
    .run()
    .await
    .expect("Should run http server got ");

    Ok(())
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

#[get("/users/{id}")]
async fn get_user(user_id: web::Path<i64>, data: web::Data<ApplicationState>) -> impl Responder {
    let unwrapped_user_id = user_id.into_inner();
    match repository::get_user(&data.database_connection_pool, unwrapped_user_id).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/users")]
async fn post_user(body: web::Json<NewUser>, data: web::Data<ApplicationState>) -> impl Responder {
    match repository::create_user(&data.database_connection_pool, body.into_inner()).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
