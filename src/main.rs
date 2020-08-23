use log::info;

use quinte::server;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Quinte Server");

    server::listen().await;
}
