use log::info;
use quinte::{notmuch, server};
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    info!("Quinte Server");

    let home = env::var("HOME").expect("$HOME is not set.");
    let db_path = format!("{}/.mail", home);

    info!("Open database at {}", db_path);
    let db = notmuch::NotmuchDb::open(&db_path).expect("Could not open the database");
    let db = Arc::new(db);

    server::listen(db).await.expect("Server failed to start");
}
