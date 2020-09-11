use anyhow::Context;
use log::info;
use quinte::{notmuch, server};
use std::env;
use std::{process, sync::Arc};

#[tokio::main]
async fn main() {
    if let Err(e) = try_main().await {
        eprintln!("{}", e);
        process::exit(1);
    }
}

async fn try_main() -> anyhow::Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    info!("Quinte Server");

    let current_dir = env::current_dir()?;
    let db_path = format!(
        "{}/testdata",
        current_dir.into_os_string().into_string().unwrap()
    );

    info!("Open database at {}", db_path);
    let db = notmuch::NotmuchDb::open(&db_path)?;
    let db = Arc::new(db);

    server::listen(db).await.context("Server failed to start")?;
    Ok(())
}
