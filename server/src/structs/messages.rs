use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ClientMessage {
    pub user: String,
    pub msg: String,
}
