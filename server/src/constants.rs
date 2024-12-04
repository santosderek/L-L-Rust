use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::protocol::Message;

// Shared state for managing clients
pub type Clients = Arc<Mutex<HashMap<String, mpsc::UnboundedSender<Message>>>>;

pub const ADDRESS: &str = "0.0.0.0";
pub const PORT: &str = "8080";
