use iron::typemap::Key;
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{PostgresConnectionManager};

pub type PostgresPool = Pool<PostgresConnectionManager>;

pub struct Database;

impl Key for Database {
    type Value = PostgresPool;
}