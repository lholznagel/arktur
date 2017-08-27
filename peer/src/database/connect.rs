use r2d2::PooledConnection;
use r2d2;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use std::fs::File;
use std::io::Read;

type Pool = r2d2::Pool<PostgresConnectionManager>;

pub fn init_database() -> Pool {
    let config_r2d2 = r2d2::Config::default();
    let manager =
        PostgresConnectionManager::new("postgres://postgres:password@localhost", TlsMode::None)
            .unwrap();

    let pool = r2d2::Pool::new(config_r2d2, manager).unwrap();

    println!("Preparing database");
    pool.get().unwrap().execute("CREATE EXTENSION IF NOT EXISTS pgcrypto;", &[]).unwrap();

    prepare_database(pool.get().unwrap(), "queries/create_block.sql");
    prepare_database(pool.get().unwrap(), "queries/create_blockchain.sql");
    println!("Done with preparing database");

    return pool;
}

fn prepare_database(db: PooledConnection<PostgresConnectionManager>, name: &str) {
    let mut file = File::open(name).unwrap();
    let mut content = String::new();

    file.read_to_string(&mut content).expect(
        "Could not read preparation script",
    );

    db.execute(content.as_str(), &[]).unwrap();
}