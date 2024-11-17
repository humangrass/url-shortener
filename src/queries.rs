use crate::errors::ShortenerError;
use crate::service::UrlShortener;
use crate::structs::{ShortLink, Slug, Stats};

/// Trait for query handlers.
pub trait QueryHandler {
    /// Returns the [`Stats`] for a specific [`ShortLink`], such as the number of redirects (clicks).
    ///
    /// [`ShortLink`]: super::ShortLink
    fn get_stats(&self, slug: Slug) -> Result<Stats, ShortenerError>;
}

impl QueryHandler for UrlShortener {
    fn get_stats(&self, slug: Slug) -> Result<Stats, ShortenerError> {
        let (links, redirects) = self.replay();

        if let Some(url) = links.get(&slug) {
            let redirects = redirects.get(&slug).copied().unwrap_or(0);
            return Ok(Stats {
                link: ShortLink {
                    slug: slug.clone(),
                    url: url.clone(),
                },
                redirects,
            });
        }

        Err(ShortenerError::SlugNotFound)
    }
}
