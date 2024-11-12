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

/// Events for Event Sourcing.
#[derive(Debug, Clone)]
pub enum Event {
    LincCreated { slug: Slug, url: Url },
    RedirectOccurred { slug: Slug },
}
