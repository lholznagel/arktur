use block::{Block, BlockApi, ResultLastBlock};
use guards::DBConnection;
use rocket_contrib::Json;
use uuid::Uuid;

#[post("/", format = "application/json", data = "<block>")]
pub fn new(db: DBConnection, block: Json<BlockApi>) {
    let last_block = get_last_block_in_chain(&db, &block.0.blockchain);
    let blockchain_signkey = get_signkey(&db, &block.0.blockchain);

    let block = Block::new(
        last_block.index + 1,
        block.0.content,
        last_block.prev,
        block.0.blockchain,
    );
    let block = block.generate_hash(String::from(blockchain_signkey));

    db.0
        .execute(
            "
        INSERT INTO block
        (blockchain, index, nonce, content, timestamp, prev, hash)
        VALUES
        ($1, $2, $3, $4, $5, $6, $7)
        ",
            &[
                &block.blockchain,
                &block.index,
                &block.nonce,
                &block.content,
                &block.timestamp,
                &block.prev,
                &block.hash,
            ],
        )
        .unwrap();
}

fn get_last_block_in_chain(db: &DBConnection, blockchain: &Uuid) -> ResultLastBlock {
    let query = db.0.query(
        "SELECT index, hash FROM block WHERE blockchain = $1 ORDER BY index DESC LIMIT 1",
        &[blockchain],
    );

    match query {
        Ok(result) => {
            if result.len() == 0 {
                ResultLastBlock {
                    index: 0,
                    prev: generate_empty_hash(),
                }
            } else {
                ResultLastBlock {
                    index: result.get(0).get(0),
                    prev: result.get(0).get(1),
                }
            }
        }
        Err(error) => panic!(error),
    }
}

/// Just gernates an empty string to get started
/// String has a length of 64 filled with 0
fn generate_empty_hash() -> String {
    let mut empty_prev = String::from("");

    for _ in 0..64 {
        empty_prev.push_str("0");
    }

    empty_prev
}

fn get_signkey(db: &DBConnection, blockchain: &Uuid) -> String {
    let query = db.0.query(
        "SELECT signkey FROM blockchain WHERE id = $1",
        &[blockchain],
    );

    match query {
        Ok(result) => {
            let signkey: String = result.get(0).get(0);
            signkey
        }
        Err(error) => panic!(error)
    }
}