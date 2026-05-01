use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions};

use crate::config::config::Config;

mod routes;
mod handlers;
mod config;

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

    println!("here is the app config jwt secret {:?}", app_config.jwt_secret);
    println!("here is the app config jwt maxage {:?}", app_config.jwt_maxage);

    sqlx::migrate!().run(&pool).await.expect("migration failed");
    
    let app = routes::create_routes();

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
