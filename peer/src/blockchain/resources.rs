use blockchain::Blockchain;
use guards::DBConnection;
use rocket_contrib::Json;

#[get("/")]
pub fn overview(db: DBConnection) {
}

#[post("/", format = "application/json", data = "<blockchain>")]
pub fn new(db: DBConnection, blockchain: Json<Blockchain>) {
    db.0
        .execute(
            "
            INSERT INTO blockchain
            (name, signkey)
            VALUES ($1, $2)
        ",
            &[&blockchain.0.name, &blockchain.0.signkey],
        )
        .unwrap();
}