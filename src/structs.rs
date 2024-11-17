use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

// TODO: maybe it's worth sorting out the traits

/// A unique string (or alias) that represents the shortened version of the URL.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
pub struct Slug(pub String);

/// The original URL that the short link points to.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Url(pub String);

/// Shortened URL representation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ShortLink {
    /// A unique string (or alias) that represents the shortened version of the URL.
    pub slug: Slug,

    /// The original URL that the short link points to.
    pub url: Url,
}

/// Statistics of the [`ShortLink`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    /// [`ShortLink`] to which this [`Stats`] are related.
    pub link: ShortLink,

    /// Count of redirects of the [`ShortLink`].
    pub redirects: u64,
}

// TODO: is it worth getting rid of it?
/// Wrapper for `EventData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub data: EventData,
}

/// Events data for Event Sourcing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventData {
    LincCreated { slug: Slug, url: Url },
    RedirectOccurred { slug: Slug },
    StatsUpdated { slug: Slug, redirects: u64 },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceState {
    pub links: HashMap<Slug, LinkData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkData {
    pub url: Url,
    pub redirects: u64,
}
