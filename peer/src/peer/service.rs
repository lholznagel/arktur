use message::{Message, Messagable, notify_new_peer};
use peer::Register;
//use guards::DBConnection;
use time::get_time;
use connections::postgres::Pool;
use uuid::Uuid;

pub fn get_all_peers(pool: &Pool) -> String {
    let mut is_first: bool = true;
    let mut result: String = String::from("[");

    for row in &pool.get().unwrap()
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

pub fn save_peer(pool: &Pool, message: &Message<Register>) {
    println!("{:?}", message);
    if !is_message_known(&pool, &message) {
        if !is_peer_known(&pool, &message) {
        pool.get().unwrap()
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

        pool.get().unwrap()
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
        notify_new_peer(&pool, &message);
    }
}

fn is_message_known(pool: &Pool, message: &Message<Register>) -> bool {
    let result = &pool.get().unwrap()
        .query(
            "SELECT id FROM messages WHERE id = $1 AND hash = $2;",
            &[&message.id, &message.hash],
        )
        .unwrap()
        .iter()
        .len();
    return if result >= &1 { true } else { false };
}

fn is_peer_known(pool: &Pool, message: &Message<Register>) -> bool {
    let result = &pool.get().unwrap()
        .query(
            "SELECT peer_id FROM peers WHERE peer_id = $1;",
            &[&message.content.peer_id],
        )
        .unwrap()
        .iter()
        .len();
    return if result >= &1 { true } else { false };
}