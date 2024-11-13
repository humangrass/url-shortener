//! ## Task Description
//!
//! The goal is to develop a backend service for shortening URLs using CQRS
//! (Command Query Responsibility Segregation) and ES (Event Sourcing)
//! approaches. The service should support the following features:
//!
//! ## Functional Requirements
//!
//! ### Creating a short link with a random slug
//!
//! The user sends a long URL, and the service returns a shortened URL with a
//! random slug.
//!
//! ### Creating a short link with a predefined slug
//!
//! The user sends a long URL along with a predefined slug, and the service
//! checks if the slug is unique. If it is unique, the service creates the short
//! link.
//!
//! ### Counting the number of redirects for the link
//!
//! - Every time a user accesses the short link, the click count should
//!   increment.
//! - The click count can be retrieved via an API.
//!
//! ### CQRS+ES Architecture
//!
//! CQRS: Commands (creating links, updating click count) are separated from
//! queries (retrieving link information).
//!
//! Event Sourcing: All state changes (link creation, click count update) must be
//! recorded as events, which can be replayed to reconstruct the system's state.
//!
//! ### Technical Requirements
//!
//! - The service must be built using CQRS and Event Sourcing approaches.
//! - The service must be possible to run in Rust Playground (so no database like
//!   Postgres is allowed)
//! - Public API already written for this task must not be changed (any change to
//!   the public API items must be considered as breaking change).

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
    let address = "localhost:3000";
    let file_path = "events.json";

    println!("Url shortener service");
    println!("Listening on http://{:?}", address);

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
            let service = service_clone.lock().unwrap();
            if let Err(e) = service.save_state(file_path) {
                eprintln!("Failed to save system state: {:?}", e);
            } else {
                println!("System state successfully saved.");
            }
        }
    });

    let app = create_router(service.clone());
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal(service.clone(), file_path)).await.unwrap()
}

async fn shutdown_signal(service: Arc<Mutex<UrlShortenerService>>, file_path: &str) {
    let _ = signal::ctrl_c().await;
    println!("Shutting down...");

    let service = service.lock().unwrap();
    if let Err(e) = service.save_state(file_path) {
        eprintln!("Failed to save system state: {:?}", e);
    } else {
        println!("System state successfully saved to {}", file_path);
    }
}
