use iron::typemap::Key;
use r2d2::{Config, Pool};
use r2d2_postgres::{PostgresConnectionManager};

pub type PostgresPool = Pool<PostgresConnectionManager>;

pub struct Database;

impl Key for Database {
    type Value = PostgresPool;
}

pub fn init_database() -> PostgresPool {
    let manager = PostgresConnectionManager::new(
        "postgres://peer1:peer1@postgres:5432",
        ::r2d2_postgres::TlsMode::None,
    ).unwrap();

    let config = Config::builder().pool_size(6).build();

    Pool::new(config, manager).unwrap()
}