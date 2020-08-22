use log::{error, info};
use std::net::SocketAddr;

use tokio::net::{TcpListener, TcpStream};

use futures_util::StreamExt;

use tokio_tungstenite::{self, accept_async};

use tungstenite::protocol::Message;

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut listener = TcpListener::bind("127.0.0.1:42337")
        .await
        .expect("Could not bind port.");

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

    while let Some(result) = stream.next().await {
        match result {
            Ok(message) => match message {
                Message::Binary(_) => info!("got data"),
                Message::Close(_) => info!("Client indicates connection close"),
                _ => info!("other"),
            },
            Err(err) => {
                error!("Couldn't receive data: {}", err);
            }
        }
    }
    info!("Closing connection from {}", remote_address);
}
