#[derive(Serialize, Deserialize)]
pub struct Config {
    pub keypair: String,
    pub node: String,
    pub user: String
}