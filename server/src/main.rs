mod endpoints;
mod models;

use actix_web::{get, middleware::Logger, web::Data, App, HttpServer, Responder};
use anyhow::Result;
use database::pg_conn::establish_pool_connection;
use endpoints::*;

#[get("/")]
async fn hello() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let pool = establish_pool_connection()?;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(hello)
            .service(user_endpoints())
            .service(chat_endpoints())
            .wrap(Logger::default())
    })
    .bind("localhost:8080")?
    .workers(2)
    .run()
    .await?;

    Ok(())
}
