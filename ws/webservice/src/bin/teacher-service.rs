use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::io;
use std::sync::Mutex;

#[path = "../db_access/mod.rs"]
mod db_access;

#[path = "../handlers/mod.rs"]
mod handlers;

#[path = "../models/mod.rs"]
mod models;

#[path = "../routers.rs"]
mod routers;

#[path = "../state.rs"]
mod state;

#[path = "../errors.rs"]
mod errors;

use routers::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "I am OK".to_string(),
        visit_count: Mutex::new(0),
        // course: Mutex::new(vec![]),
        db: db_pool,
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                errors::MyError::InvalidTnput("Please provide valid Json input".to_string()).into()
            }))
            .configure(general_routes)
            .configure(course_routes)
            .configure(teacher_routes)
    };

    // 0.0.0.0 表示监听所有ip， 127.0.0.1和localhost 表示只能被本机ip访问
    HttpServer::new(app).bind("0.0.0.0:3000")?.run().await
}
