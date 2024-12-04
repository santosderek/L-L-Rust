extern crate server;

use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use tracing::{debug, error, info, Level};
use tracing_subscriber;

use server::constants::{Clients, ADDRESS, PORT};
use server::structs::messages::ClientMessage;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    info!("Starting WebSocket server");
    // Initialize client store
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    // Start the TCP listener
    let addr = format!("{}:{}", ADDRESS, PORT);
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    debug!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        info!("Client connected from {}!", stream.peer_addr().unwrap());
        let clients = clients.clone();
        tokio::spawn(async move {
            if let Ok(ws_stream) = accept_async(stream).await {
                client_connected(ws_stream, clients).await;
            }
        });
    }
}

async fn client_connected(
    ws_stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
    clients: Clients,
) {
    // Generate a unique ID for the client
    let id = Arc::new(uuid::Uuid::new_v4().to_string());

    // Create channel to send messages to this client
    let (tx, mut rx) = mpsc::unbounded_channel();

    // NOTE: We are adding the transmitter to the clients list.
    // We want to echo the message back to all clients except the sender.
    clients.lock().unwrap().insert(id.clone().to_string(), tx);

    // Spawn a task to send messages to the client
    let (mut ws_sink, mut ws_stream) = ws_stream.split();
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            // Validate Message is in correct format
            match serde_json::from_str::<ClientMessage>(&msg.to_string()) {
                Ok(client_message) => {
                    info!(
                        "Received message from {}: {}",
                        client_message.user, client_message.msg
                    );

                    if ws_sink.send(msg).await.is_err() {
                        error!(
                            "Error sending message '{}'  from user '{}'",
                            client_message.msg, client_message.user
                        );
                        // break;
                        continue;
                    }
                }
                Err(e) => {
                    error!("Error deserializing message: {}", e);
                    continue;
                }
            };
        }
    });

    let id = id.clone();
    // Receive messages from the client
    while let Some(Ok(ref msg)) = ws_stream.next().await {
        handle_message(&msg, &id, &clients).await;
    }

    // Remove client from list when done
    let id = id.clone().to_string();
    clients.lock().unwrap().remove(&id);
}

async fn handle_message(msg: &Message, sender_id: &String, clients: &Clients) {
    if let Message::Text(text) = msg {
        // Broadcast the message to all other clients
        for (id, tx) in clients.lock().unwrap().iter() {
            if id != sender_id {
                let _ = tx.send(Message::Text(text.clone()));
            }
        }
    }
}
