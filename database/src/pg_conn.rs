use dotenvy::dotenv;
use std::env;

use anyhow::Result;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type PooledPgConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn database_uri() -> Result<String> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Ok(database_url)
}

pub fn establish_connection() -> Result<PgConnection> {
    let uri = database_uri()?;

    let conn = PgConnection::establish(&uri)?;
    Ok(conn)
}

pub fn establish_pool_connection() -> Result<Pool> {
    let uri = database_uri()?;

    let manager = ConnectionManager::<PgConnection>::new(uri);
    let pool = r2d2::Pool::builder().build(manager)?;
    Ok(pool)
}
