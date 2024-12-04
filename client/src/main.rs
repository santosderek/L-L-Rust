extern crate client;

use futures_util::{SinkExt, StreamExt};
use std::io::{self, Write};
use tokio::select;
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

use client::structs::messages::{ClientMessage, ServerErrorMessage};

const WS_URL: &str = "ws://localhost:8080";
const username: &str = "derek";

#[tokio::main]
async fn main() {
    let url = Url::parse(WS_URL).expect("Invalid URL").to_string();

    let (ws_stream, _) = connect_async(&url)
        .await
        .expect("Failed to connect to WebSocket server");

    println!("Connected to server");

    let (mut write, mut read) = ws_stream.split();
    let (tx, mut rx) = mpsc::channel::<String>(100);

    // Task for reading user input from stdin
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        let mut input = String::new();
        loop {
            input.clear();
            print!("> ");
            io::stdout().flush().unwrap();
            if io::stdin().read_line(&mut input).is_err() {
                eprintln!("Failed to read from stdin");
                break;
            }
            let msg = input.trim().to_string();
            if tx_clone.send(msg).await.is_err() {
                eprintln!("Failed to send user input");
                break;
            }
        }
    });

    loop {
        select! {
            // Send user input to the WebSocket server
            Some(msg) = rx.recv() => {
                let msg = ClientMessage {
                    user: username.to_string(),
                    msg,
                };
                if write.send(Message::Text(serde_json::to_string(&msg).unwrap())).await.is_err() {
                    eprintln!("Failed to send message");
                    break;
                }
            }
            // Read messages from the WebSocket server
            Some(Ok(message)) = read.next() => {

                match serde_json::from_str::<ServerErrorMessage>(&message.to_string()) {
                    Ok(server_error) => {
                        eprintln!("\rServer error: {}", server_error.error);
                        print!("> ");
                        continue;
                    }
                    Err(_) => {
                        // eprintln!("Failed to parse server message");
                    }
                }

                match serde_json::from_str::<ClientMessage>(&message.to_string()) {
                    Ok(client_message) => {
                        println!("\r@{}: {}", client_message.user, client_message.msg);
                        print!("> ");
                    }
                    Err(_) => {
                        eprintln!("Failed to parse client message");
                        continue;
                    }
                }

            }
            else => {
                break;
            }
        }
    }

    println!("Exiting.");
}

