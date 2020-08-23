use log::{error, info};
use std::net::SocketAddr;

use tokio::net::{TcpListener, TcpStream};

use futures_util::{SinkExt, StreamExt};

use tokio_tungstenite::{self, accept_async};

use tungstenite::protocol::Message;

use serde::{Deserialize, Serialize};

/// Something that we send upon established connections so that the other side knows what they're
/// dealing with
#[derive(Serialize, Deserialize, Debug)]
struct Greeting {
    version: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct QuinteMessage {
    kind: String,
    correlation_id: String,
}

pub async fn listen() {
    let mut listener = TcpListener::bind("127.0.0.1:42337")
        .await
        .expect("Could not bind port.");

    info!("Waiting for connections");

    while let Ok((stream, remote_address)) = listener.accept().await {
        info!("Connection attempt: {}", remote_address);
        tokio::spawn(accept_connection(stream, remote_address));
    }
}

async fn accept_connection(tcp_stream: TcpStream, remote_address: SocketAddr) {
    let mut stream = accept_async(tcp_stream).await.expect(&format!(
        "Websocket handshake failed for {}",
        remote_address
    ));
    info!("Established connection from {}", remote_address);

    stream
        .send(Message::Text(
            serde_json::to_string(&Greeting { version: 1 }).unwrap(),
        ))
        .await
        .expect("Could not send status greeting");

    while let Some(result) = stream.next().await {
        match result {
            Ok(message) => match message {
                Message::Text(payload) => {
                    tokio::spawn(async move {
                        handle_message(payload).await;
                    });
                }
                Message::Close(_) => info!("Client indicates connection close"),
                _ => error!("Client {} sent unhandled message format", remote_address),
            },
            Err(err) => {
                error!("Couldn't receive data: {}", err);
            }
        }
    }
    info!("Closing connection from {}", remote_address);
}

async fn handle_message(raw_message: String) {
    info!("Request: {}", raw_message);
    let msg: QuinteMessage = serde_json::from_str(&raw_message).expect("Could not parse message");
}
