use config::Database as DatabaseConfig;
use iron::typemap::Key;
use r2d2::{Config, Pool};
use r2d2_postgres::{PostgresConnectionManager, TlsMode};

pub type PostgresPool = Pool<PostgresConnectionManager>;

pub struct Database;

impl Key for Database {
    type Value = PostgresPool;
}

pub fn init_database(config: &DatabaseConfig) -> PostgresPool {
    let manager = PostgresConnectionManager::new(
        format!("postgres://{}:{}@{}:{}", config.user, config.password, config.address, config.port),
        TlsMode::None,
    ).unwrap();

    let config = Config::builder().pool_size(6).build();

    Pool::new(config, manager).unwrap()
}