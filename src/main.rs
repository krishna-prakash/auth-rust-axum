use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions};

use crate::{config::config::Config, database::{DBClient}};

mod routes;
mod handlers;
mod config;
mod database;
mod models;
mod dtos;

#[derive(Debug, Clone)]
struct AppState {
    config: Config,
    db_client: DBClient,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app_config = Config::init();
    let addr = format!("0.0.0.0:{}", app_config.port);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&app_config.db_url)
        .await
        .expect("db connection failed");

    let db = DBClient::new(pool);

    println!("here is the app config jwt secret {:?}", app_config.jwt_secret);
    println!("here is the app config jwt maxage {:?}", app_config.jwt_maxage);

    let app_state = AppState {
        config: app_config,
        db_client: db.clone(),
    };

    sqlx::migrate!().run(&db.pool).await.expect("migration failed");

    let app = routes::create_routes().with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
