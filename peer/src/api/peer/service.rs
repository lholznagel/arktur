use api::peer::Register;
use guards::DBConnection;
use time::get_time;

pub struct PeerService {}

impl PeerService {
    pub fn save_peer(db: &DBConnection, message: &Register) {
        db.0.execute(
            "
        INSERT INTO peers
        (name, registered_at, last_seen)
        VALUES
        ($1, $2, $2)
        ",
            &[&message.name, &get_time().sec],
        ).unwrap();
    }
}