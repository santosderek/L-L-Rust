use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ClientMessage {
    pub user: String,
    pub msg: String,
}

#[derive(Serialize, Deserialize)]
pub struct ServerErrorMessage {
    pub code: u16,
    pub error: String,
}
