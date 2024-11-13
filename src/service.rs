use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufReader, BufWriter, Write};
use std::sync::{Arc, Mutex};
use crate::structs::{Slug, Url, EventData, Event, ServiceState, LinkData};

pub type SharedService = Arc<Mutex<UrlShortenerService>>;

/// CQRS and Event Sourcing-based service implementation.
pub struct UrlShortenerService {
    pub events: Vec<Event>,
}

impl UrlShortenerService {
    /// Creates a new instance of the service.
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    /// Saves an event to the file.
    pub fn save_state(&self, file_path: &str) -> io::Result<()> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_path)?;

        let mut writer = BufWriter::new(file);

        let state = self.build_service_state();

        let serialized = serde_json::to_string(&state)?;
        writeln!(writer, "{}", serialized)?;
        Ok(())
    }

    /// Loads all events from the file.
    pub fn load_state(file_path: &str) -> io::Result<ServiceState> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let state: ServiceState = serde_json::from_reader(reader)?;
        Ok(state)
    }

    fn build_service_state(&self) -> ServiceState {
        let (links, redirects) = self.replay();

        let mut state_links = HashMap::new();
        for (slug, url) in links {
            let redirect_count = redirects.get(&slug).cloned().unwrap_or(0);
            state_links.insert(slug, LinkData { url, redirects: redirect_count });
        }

        ServiceState { links: state_links }
    }

    pub fn from_state(state: ServiceState) -> Self {
        let mut events = Vec::new();

        for (slug, link_data) in state.links {
            events.push(Event {
                data: EventData::LincCreated {
                    slug: slug.clone(),
                    url: link_data.url,
                },
            });

            for _ in 0..link_data.redirects {
                events.push(Event {
                    data: EventData::RedirectOccurred {
                        slug: slug.clone(),
                    },
                });
            }
        }

        Self { events }
    }

    /// Reconstructs the state from loaded events
    pub fn replay(&self) -> (HashMap<Slug, Url>, HashMap<Slug, u64>) {
        let mut links = HashMap::new();
        let mut redirects = HashMap::new();

        for event in &self.events {
            match &event.data {
                EventData::LincCreated { slug, url } => {
                    links.insert(slug.clone(), url.clone());
                    redirects.insert(slug.clone(), 0);
                }
                EventData::RedirectOccurred { slug } => {
                    if let Some(count) = redirects.get_mut(slug) {
                        *count += 1;
                    }
                }
                EventData::StatsUpdated { slug, redirects: new_count } => {
                    redirects.insert(slug.clone(), *new_count);
                }
            }
        }
        (links, redirects)
    }
}
