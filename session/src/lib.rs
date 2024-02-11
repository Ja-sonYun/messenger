use actix_session::{storage::RedisActorSessionStore, SessionMiddleware};
use actix_web::cookie::Key;

pub fn session_middleware(url: &str, key: &Key) -> SessionMiddleware<RedisActorSessionStore> {
    SessionMiddleware::builder(RedisActorSessionStore::new(url), key.clone()).build()
}
