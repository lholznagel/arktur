use message::{Message, Messagable, notify_new_peer};
use peer::Register;
use guards::DBConnection;
use time::get_time;
use uuid::Uuid;

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

pub fn save_peer(db: &DBConnection, message: &Message<Register>) {
    if !is_message_known(&db, &message) {

        if !is_peer_known(&db, &message) {
        db.0
            .execute(
                "
                    INSERT INTO peers
                    (address, name, port, peer_id, notify_on_change, registered_at, last_seen)
                    VALUES
                    ($1, $2, $3, $4, $5, $6, $6)
                ",
                &[
                    &message.content.address,
                    &message.content.name,
                    &message.content.port,
                    &message.content.peer_id,
                    &message.content.notify_on_change,
                    &get_time().sec,
                ],
            )
            .unwrap();
        }

        db.0
            .execute(
                "
                    INSERT INTO messages
                    (id, hash)
                    VALUES
                    ($1, $2)
                ",
                &[&message.id, &message.hash],
            )
            .unwrap();
        notify_new_peer(&db, &message);
    }
}

fn is_message_known(db: &DBConnection, message: &Message<Register>) -> bool {
    let result = &db.0
        .query(
            "SELECT id FROM messages WHERE id = $1 AND hash = $2;",
            &[&message.id, &message.hash],
        )
        .unwrap()
        .iter()
        .len();
    return if result >= &1 { true } else { false };
}

fn is_peer_known(db: &DBConnection, message: &Message<Register>) -> bool {
    let result = &db.0
        .query(
            "SELECT peer_id FROM peers WHERE peer_id = $1;",
            &[&message.content.peer_id],
        )
        .unwrap()
        .iter()
        .len();
    return if result >= &1 { true } else { false };
}