use peer::{Messagable, Register};
use guards::DBConnection;
use time::get_time;

pub fn get_all_peers(db: &DBConnection) -> String {
    let mut is_first: bool = true;
    let mut result: String = String::from("[");

    for row in &db.0
        .query(
            "SELECT address, name, port, peer_id
            FROM peers",
            &[],
        )
        .unwrap()
    {
        if !is_first {
            result.push_str(",");
        }

        let register = Register {
            address: row.get(0),
            name: row.get(1),
            port: row.get(2),
            peer_id: row.get(3)
        };

        result.push_str(register.as_json().as_str().unwrap());
        is_first = false;
    }

    result.push_str("]");
    result
}

pub fn save_peer(db: &DBConnection, message: &Register) {
    db.0.execute(
        "
    INSERT INTO peers
    (address, name, port, peer_id, registered_at, last_seen)
    VALUES
    ($1, $2, $3, $4, $5, $5)
    ",
        &[&message.address, &message.name, &message.port, &message.peer_id, &get_time().sec],
    ).unwrap();
}