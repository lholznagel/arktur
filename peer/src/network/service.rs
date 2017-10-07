use connection::PostgresPooledConnection;
use network::model::Node;

pub struct NetworkService {
    connection: PostgresPooledConnection
}

impl NetworkService {
    pub fn new(pool: PostgresPooledConnection) -> Self {
        NetworkService {
            connection: pool
        }
    }

    pub fn get_nodes(self) -> Vec<Node> {
        let mut nodes = Vec::new();

        for row in &self.connection.query("SELECT name, node_id, registered_at FROM nodes", &[]).unwrap() {
            let node = Node {
                name: row.get(0),
                node_id: row.get(1),
                registered_at: row.get(2)
            };

            nodes.push(node);
        }

        nodes
    }
}