use std::io::{self, Write};
use tokio::sync::mpsc;
use tokio::task;
use tokio_stream::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

#[tokio::main]
async fn main() {
    let url = Url::parse("ws://localhost:8080").expect("Invalid URL");

    let (ws_stream, _) = connect_async(url)
        .await
        .expect("Failed to connect to WebSocket server");
    println!("Connected to server");

    let (mut write, mut read) = ws_stream.split();

    let (tx, mut rx) = mpsc::channel::<String>(100);

    // Task for reading messages from the server
    let read_task = task::spawn(async move {
        while let Some(Ok(message)) = read.next().await {
            if let Message::Text(text) = message {
                println!("Received: {}", text);
            }
        }
    });

    // Task for sending user input to the server
    let write_task = task::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Err(e) = write.send(Message::Text(msg)).await {
                eprintln!("Error sending message: {}", e);
                break;
            }
        }
    });

    // Task for reading user input from stdin
    let tx_clone = tx.clone();
    let input_task = task::spawn(async move {
        let mut input = String::new();
        loop {
            input.clear();
            print!("> ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read from stdin");
            let msg = input.trim().to_string();
            if tx_clone.send(msg).await.is_err() {
                eprintln!("Failed to send user input");
                break;
            }
        }
    });

    tokio::select! {
        _ = read_task => {}
        _ = write_task => {}
        _ = input_task => {}
    }
}

