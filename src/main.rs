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

use crate::commands::CommandHandler;
use crate::errors::ShortenerError;
use crate::queries::QueryHandler;
use crate::service::UrlShortenerService;
use crate::structs::{Slug, Url};

fn main() {
    let mut service = UrlShortenerService::new();

    let url = Url("https://example.com".to_string());
    let slug = Slug("example".to_string());

    let link = service
        .handle_create_short_link(url.clone(), Some(slug.clone()))
        .unwrap();
    println!("Created Short Link: {:?}", link);

    service.handle_redirect(slug.clone()).unwrap();
    let stats = service.get_stats(slug.clone()).unwrap();
    println!("Stats after one redirect: {:?}", stats);

    let duplicate = service.handle_create_short_link(url.clone(), Some(slug));
    assert_eq!(duplicate, Err(ShortenerError::SlugAlreadyInUse));
}
