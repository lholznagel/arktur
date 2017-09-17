use message::Messagable;
use peer::Register;
use guards::DBConnection;
use time::get_time;

pub fn get_all_peers(db: &DBConnection) -> String {
    let mut is_first: bool = true;
    let mut result: String = String::from("[");

    for row in &db.0
        .query(
            "SELECT address, name, port, peer_id, notify_on_change
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
            peer_id: row.get(3),
            notify_on_change: row.get(4),
        };

        result.push_str(register.as_json().to_string().as_str());
        is_first = false;
    }

    result.push_str("]");
    result
}

pub fn save_peer(db: &DBConnection, message: &Register) {
    db.0.execute(
        "
    INSERT INTO peers
    (address, name, port, peer_id, notify_on_change, registered_at, last_seen)
    VALUES
    ($1, $2, $3, $4, $5, $6, $6)
    ",
        &[&message.address, &message.name, &message.port, &message.peer_id, &message.notify_on_change, &get_time().sec],
    ).unwrap();
}