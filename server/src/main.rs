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
use server::structs::messages::{ClientMessage, ServerErrorMessage};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE) // Could do TRACE for full verbosity
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
    let (mut ws_write, mut ws_read) = ws_stream.split();
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            debug!("Sending message to client: {:?}", msg);
            // Validate Message is in correct format
            match serde_json::from_str::<ClientMessage>(&msg.to_string()) {
                Ok(client_message) => {
                    info!(
                        "Received message from {}: {}",
                        client_message.user, client_message.msg
                    );

                    if ws_write.send(msg).await.is_err() {
                        error!(
                            "Error sending message '{}'  from user '{}'",
                            client_message.msg, client_message.user
                        );
                        continue;
                    }
                }
                Err(e) => {
                    error!("Error deserializing message: {}", e);

                    let response = ws_write
                        .send(Message::Text(
                            serde_json::to_string(&ServerErrorMessage {
                                code: 400,
                                error: "Invalid message format".to_string(),
                            })
                            .unwrap(),
                        ))
                        .await;
                    if response.is_err() {
                        error!("Error sending message to client");
                        break;
                    }
                }
            };
        }
    });

    let id = id.clone();
    // Receive messages from the client
    while let Some(Ok(ref msg)) = ws_read.next().await {
        debug!("Received message from client: {:?}", msg);
        handle_message(&msg, &id, &clients).await;
    }

    // Remove client from list when done
    let id = id.clone().to_string();
    clients.lock().unwrap().remove(&id);
}

async fn handle_message(msg: &Message, sender_id: &String, clients: &Clients) {
    match msg {
        Message::Close(_) => {
            debug!("Client disconnected: {}", sender_id);
        }

        Message::Text(text) => {
            debug!("Received text message: {}", text);

            // Broadcast the message to all other clients
            debug!("Broadcasting message to all clients: {}", text);
            for (id, tx) in clients.lock().unwrap().iter() {
                // if id != sender_id {
                //     let _ = tx.send(Message::Text(text.clone()));
                // }
                let _ = tx.send(Message::Text(text.clone()));
            }
        }

        _ => {
            error!("Received unsupported message type");
        }
    }
}
