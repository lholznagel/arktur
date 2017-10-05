use connection::PostgresPooledConnection;
use network::model::Peer;

pub struct NetworkService {
    connection: PostgresPooledConnection
}

impl NetworkService {
    pub fn new(pool: PostgresPooledConnection) -> Self {
        NetworkService {
            connection: pool
        }
    }

    pub fn get_peers(self) -> Vec<Peer> {
        let mut peers = Vec::new();

        for row in &self.connection.query("SELECT name FROM peers", &[]).unwrap() {
            let peer = Peer {
                name: row.get(0)
            };

            peers.push(peer);
        }

        peers
    }
}