/// All possible errors of the [`UrlShortenerService`].
#[derive(Debug, PartialEq)]
pub enum ShortenerError {
    /// This error occurs when an invalid [`Url`] is provided for shortening.
    InvalidUrl,

    /// This error occurs when an attempt is made to use a slug (custom alias) that already exists.
    SlugAlreadyInUse,

    /// This error occurs when the provided [`Slug`] does not map to any existing short link.
    SlugNotFound,
}

#[derive(Debug)]
pub enum ServiceError {
    LockError,
    ProcessingError(String),
    NotFound,
    BadRequest,
}