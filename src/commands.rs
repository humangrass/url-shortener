use crate::errors::ShortenerError;
use crate::service::UrlShortenerService;
use crate::structs::{Event, ShortLink, Slug, Url};

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

impl CommandHandler for UrlShortenerService {
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
        self.events.push(Event::LincCreated {
            slug: slug.clone(),
            url: url.clone(),
        });

        Ok(ShortLink { slug, url })
    }

    fn handle_redirect(
        &mut self,
        slug: Slug,
    ) -> Result<ShortLink, ShortenerError> {
        let (links, _) = self.replay();
        if let Some(url) = links.get(&slug) {
            self.events.push(Event::RedirectOccurred { slug: slug.clone() });
            return Ok(ShortLink {
                slug,
                url: url.clone(),
            });
        }

        Err(ShortenerError::SlugNotFound)
    }
}
