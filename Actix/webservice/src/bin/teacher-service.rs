use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use std::sync::Mutex;
use std::{io, env};
use dotenv::dotenv;

#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../dbaccess/mod.rs"]
mod dbaccess;
#[path = "../models/mod.rs"]
mod models;
#[path = "../routers.rs"]
mod routers;
#[path = "../errors.rs"]
mod errors;
#[path = "../state.rs"]
mod state;

use routers::*;
use state::AppState;
use crate::errors::MyError;

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
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                MyError::InvalidInput("Player provide valid Json input".to_string()).into()
            }))
            .configure(general_routes)
            .configure(course_routes)
            .configure(teacher_routes)
    };
    println!("Service is running");
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}