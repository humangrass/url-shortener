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
use tokio::signal;
use crate::api::create_router;
use crate::service::UrlShortenerService;

#[tokio::main]
async fn main() {
    println!("Url shortener service");
    println!("Listening on http://localhost:3000");

    let service = Arc::new(Mutex::new(UrlShortenerService::new()));
    let app = create_router(service);
    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();

    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await.unwrap()
}

async fn shutdown_signal() {
    let _ = signal::ctrl_c().await;
    println!("Shutting down...");
}
