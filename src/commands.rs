use crate::errors::ShortenerError;
use crate::service::UrlShortener;
use crate::structs::{Event, EventData, ShortLink, Slug, Url};

/// Trait for command handlers.
pub trait CommandHandler {
    /// Creates a new short link. It accepts the original url and an optional [`Slug`].
    /// If a [`Slug`] is not provided, the service will generate one.
    /// Returns the newly created [`ShortLink`].
    ///
    /// ## Errors
    ///
    /// See [`ShortenerError`].
    fn handle_create_short_link(
        &mut self,
        url: Url,
        slug: Option<Slug>,
    ) -> Result<ShortLink, ShortenerError>;

    /// Processes a redirection by [`Slug`], returning the associated
    /// [`ShortLink`] or a [`ShortenerError`].
    fn handle_redirect(
        &mut self,
        slug: Slug,
    ) -> Result<ShortLink, ShortenerError>;
}

impl CommandHandler for UrlShortener {
    fn handle_create_short_link(
        &mut self,
        url: Url,
        slug: Option<Slug>,
    ) -> Result<ShortLink, ShortenerError> {
        // Validate URL.
        if url.0.is_empty() || !url.0.starts_with("http") {
            return Err(ShortenerError::InvalidUrl);
        }

        // Replay state to check slug uniqueness.
        let (links, _) = self.replay();
        let slug = slug.unwrap_or_else(|| Slug(nanoid::nanoid!()));

        if links.contains_key(&slug) {
            return Err(ShortenerError::SlugAlreadyInUse);
        }

        // Record the event
        let event = Event {
            data: EventData::LincCreated {
                slug: slug.clone(),
                url: url.clone(),
            },
        };
        self.events.push(event);

        Ok(ShortLink { slug, url })
    }

    fn handle_redirect(
        &mut self,
        slug: Slug,
    ) -> Result<ShortLink, ShortenerError> {
        let (links, _) = self.replay();
        if let Some(url) = links.get(&slug) {
            let event_redirect = Event {
                data: EventData::RedirectOccurred { slug: slug.clone() },
            };
            self.events.push(event_redirect);

            let redirects = self.replay().1.get(&slug).copied().unwrap_or(0);
            let event_stats = Event {
                data: EventData::StatsUpdated {
                    slug: slug.clone(),
                    redirects,
                },
            };
            self.events.push(event_stats);

            return Ok(ShortLink {
                slug,
                url: url.clone(),
            });
        }

        Err(ShortenerError::SlugNotFound)
    }
}
