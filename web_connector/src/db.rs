//! Db executor actor
use std::io;
use uuid;
use actix_web::*;
use actix::prelude::*;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;


/// This is db executor actor. We are going to run 3 of them in parallel.
pub struct DbExecutor(pub Pool<SqliteConnectionManager>);

/// This is only message that this actor can handle, but it is easy to extend number of
/// messages.
pub struct InsertBlock {
    pub index: i64,
    /// Timestamp the block was created
    pub timestamp: i64,
    /// Nonce for this block
    pub nonce: i64,
    /// Hash of the previous block
    pub prev: String,
    /// Hash of this block
    pub hash: String,
    /// Content of the block
    pub content: String
}

impl Message for InsertBlock {
    type Result = Result<(), io::Error>;
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<InsertBlock> for DbExecutor {
    type Result = Result<(), io::Error>;

    fn handle(&mut self, msg: InsertBlock, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().unwrap();

        let uuid = format!("{}", uuid::Uuid::new_v4());
        conn.execute("INSERT INTO blocks (id, index, timestamp, nonce, prev, hash, content) 
                      VALUES ($1, $2, $3, $4, $5, $6, $7)", 
                      &[&uuid, &msg.index, &msg.timestamp, &msg.nonce, &msg.prev, &msg.hash, &msg.content])
                      .map_err(|_| io::Error::new(io::ErrorKind::Other, "db error"))?;
        Ok(())
    }
}