use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Node {
    pub name: String,
    pub node_id: Uuid,
    pub registered_at: i64
}