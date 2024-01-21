use crate::models::user::User;
use actix_web::dev::HttpServiceFactory;
use actix_web::{get, web, HttpResponse};
use database::pg_conn::Pool;
use database::schema::users;
use diesel::prelude::*;

pub fn user_endpoints() -> impl HttpServiceFactory {
    web::scope("/users").service(index)
}

#[get("/")]
async fn index(pool: web::Data<Pool>) -> HttpResponse {
    let mut conn = pool
        .get()
        .expect("couldn't get driver connection from pool");

    let results: Vec<User> =
        web::block(move || users::table.select(User::as_select()).load(&mut conn))
            .await
            .map_err(|e| {
                eprintln!("Error: {}", e);
                HttpResponse::InternalServerError().finish()
            })
            .expect("Error in load posts")
            .expect("Error in load posts");

    HttpResponse::Ok().json(results)
}
