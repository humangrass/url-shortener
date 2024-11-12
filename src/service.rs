use std::collections::HashMap;
use crate::structs::{Slug, Url, Event};

/// CQRS and Event Sourcing-based service implementation.
pub struct UrlShortenerService {
    pub events: Vec<Event>,
}

impl UrlShortenerService {
    /// Creates a new instance of the service.
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub(crate) fn replay(&self) -> (HashMap<Slug, Url>, HashMap<Slug, u64>) {
        let mut links = HashMap::new();
        let mut redirects = HashMap::new();

        for event in &self.events {
            match event {
                Event::LincCreated { slug, url } => {
                    links.insert(slug.clone(), url.clone());
                    redirects.insert(slug.clone(), 0);
                }
                Event::RedirectOccurred { slug } => {
                    if let Some(count) = redirects.get_mut(slug) {
                        *count += 1;
                    }
                }
            }
        }
        (links, redirects)
    }
}
