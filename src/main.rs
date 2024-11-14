mod commands;
mod structs;
mod queries;
mod service;
mod errors;
mod api;

use std::sync::{Arc, Mutex};
use tokio::{signal, time};
use std::time::Duration;
use crate::api::create_router;
use crate::service::UrlShortenerService;

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("Fatal error occurred: {}", err);
        std::process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    // TODO: should be moved to cli, but will do it later
    let address = "localhost:3000";
    let file_path = "events.json";

    println!("Url shortener service");
    println!("Listening on http://{}", address);

    let service = {
        match UrlShortenerService::load_state(file_path) {
            Ok(state) => {
                println!("Saved state found. Importing...");
                let service = Arc::new(Mutex::new(UrlShortenerService::from_state(state)));
                println!("Done!");
                service
            }
            Err(_) => {
                println!("No saved state found. Starting fresh...");
                Arc::new(Mutex::new(UrlShortenerService::new()))
            }
        }
    };

    let service_clone = service.clone();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(300));
        loop {
            interval.tick().await;
            save_service_state(&service_clone, file_path);
        }
    });

    let app = create_router(service.clone());
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to bind to {}: {}", address, e))?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(service.clone(), file_path))
        .await
        .map_err(|e| anyhow::anyhow!("Failed to start the server: {}", e))?;

    Ok(())
}

async fn shutdown_signal(service: Arc<Mutex<UrlShortenerService>>, file_path: &str) {
    let _ = signal::ctrl_c().await;
    println!("Shutting down...");

    save_service_state(&service, file_path);
}

// TODO: would take out
fn save_service_state(service: &Arc<Mutex<UrlShortenerService>>, file_path: &str) {
    match service.lock() {
        Ok(service) => {
            if let Err(e) = service.save_state(file_path) {
                eprintln!("Failed to save system state: {:?}", e);
            } else {
                println!("System state successfully saved.");
            }
        }
        Err(err) => {
            eprintln!("Failed to acquire lock on service: {:?}", err);
        }
    }
}
