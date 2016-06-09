extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate iron;
extern crate persistent;

use self::r2d2_postgres::{SslMode, PostgresConnectionManager};
use self::iron::prelude::Chain;
use self::iron::typemap::Key;
use self::persistent::Read;
use std::sync::Arc;

pub type Pool = r2d2::Pool<r2d2_postgres::PostgresConnectionManager<>>;
pub type Connection = r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>;
pub type ArcPool = Arc<Pool>;
pub struct Database;

impl Key for Database {
    type Value = Pool;
}

pub fn connect() -> Result<Pool, r2d2::InitializationError> {
    let database_url = env!("DATABASE_URL");

    let config = r2d2::Config::default();
    let manager = PostgresConnectionManager::new(database_url, SslMode::None).unwrap();
    r2d2::Pool::new(config, manager)
}

pub fn attach(pool: Pool, chain: &mut Chain) {
    chain.link(Read::<Database>::both(pool));
}
