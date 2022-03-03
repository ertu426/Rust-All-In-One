use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use std::sync::Mutex;
use std::{io, env};
use dotenv::dotenv;

#[path = "../db_access.rs"]
mod db_access;
#[path = "../handlers.rs"]
mod handlers;
#[path = "../routers.rs"]
mod routers;
#[path = "../errors.rs"]
mod errors;
#[path = "../models.rs"]
mod models;
#[path = "../state.rs"]
mod state;

use routers::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL 没有设置");

    let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),
        // courses: Mutex::new(vec![]),
        db: db_pool,
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };
    println!("Service is running");
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}