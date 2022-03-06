#[path = "../mod.rs"]
mod webapp;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use tera::Tera;
use webapp::{errors, handlers, models, routers::app_config};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host_port = env::var("HOST_PORT").expect("HOST: host address is not set in .env file");
    println!("Listening on:{}", &host_port);
    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*")).unwrap();
        App::new()
            .app_data(web::Data::new(tera))
            .configure(app_config)
    })
    .bind(&host_port)?
    .run()
    .await
}
