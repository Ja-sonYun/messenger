mod endpoints;
mod models;
use std::sync::Arc;

use actix_web::cookie::Key;
use actix_web::{get, middleware::Logger, web::Data, App, HttpServer, Responder};
use anyhow::Result;
use database::pg_conn::establish_pool_connection;
use endpoints::*;
use session::session_middleware;

#[get("/")]
async fn hello() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Global states
    let pool = establish_pool_connection()?;
    let broadcaster = Broadcaster::create();
    let private_key = Key::generate();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(Data::from(Arc::clone(&broadcaster)))
            .wrap(session_middleware("127.0.0.1:6379", &private_key))
            .service(hello)
            .service(user_endpoints())
            .service(chat_endpoints())
            .wrap(Logger::default())
    })
    .bind("localhost:8080")?
    .workers(4)
    .run()
    .await?;

    Ok(())
}
