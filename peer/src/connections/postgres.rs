use config::Database;
use r2d2;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};

pub type Pool = r2d2::Pool<PostgresConnectionManager>;

pub fn init(config: &Database) -> Pool {
    let config_r2d2 = r2d2::Config::default();

    let connection_string = format!("postgres://{}:{}@{}:{}", config.user, config.password, config.address, config.port);
    let manager =
        PostgresConnectionManager::new(connection_string, TlsMode::None)
            .unwrap();

    let pool = r2d2::Pool::new(config_r2d2, manager).unwrap();
    pool.get().unwrap().execute("DELETE FROM peers;", &[]).unwrap();
    pool
}