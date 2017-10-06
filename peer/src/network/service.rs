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

        for row in &self.connection.query("SELECT name FROM nodes", &[]).unwrap() {
            let node = Node {
                name: row.get(0)
            };

            nodes.push(node);
        }

        nodes
    }
}