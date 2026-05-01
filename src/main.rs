use std::env;

use dotenvy::dotenv;
use sqlx::{pool::PoolConnection, postgres::PgPoolOptions};

mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("DB_URL").expect("not able to load db  url");
    println!("This is {} database url", db_url);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("db connection failed");

    sqlx::migrate!().run(&pool).await.expect("migration failed");
    
    let app = routes::create_routes();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
