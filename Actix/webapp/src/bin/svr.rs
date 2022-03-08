#[path = "../mod.rs"]
mod wa;

use wa::{errors, handlers, models, routers};

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use routers::app_config;
use std::env;

use tera::Tera;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host_url = env::var("HOST_PORT").expect("HOST_URL not set");
    println!("Listening on: {}", &host_url);

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*")).unwrap();
        App::new().app_data(web::Data::new(tera)).configure(app_config)
    })
        .bind(&host_url)?
        .run()
        .await
}