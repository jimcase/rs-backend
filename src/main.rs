use actix_web::{web::Data, App, HttpServer};
use sqlx::SqlitePool;
mod handlers;
mod models;
mod routes;
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db_pool.clone()))
            .configure(routes::config_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
