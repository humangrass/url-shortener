use axum::{
    extract::{Path, State},
    Json, Router,
    routing::{get, post},
};
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};
use crate::commands::CommandHandler;
use crate::queries::QueryHandler;
use crate::service::SharedUrlShortener;
use crate::structs::{ShortLink, Slug, Url};

/// Payload for creating a short link with a random slug
#[derive(serde::Deserialize, utoipa::ToSchema)]
pub struct CreateShortLinkRequest {
    pub url: String,
}

/// Request payload for creating a short link with a predefined slug
#[derive(serde::Deserialize, utoipa::ToSchema)]
pub struct CreateShortLinkWithSlugRequest {
    pub url: String,
    pub slug: String,
}

/// Response for stats
#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct StatsResponse {
    pub slug: String,
    pub url: String,
    pub redirects: u64,
}

/// Create a short link with a random slug
#[utoipa::path(
    post,
    path = "/shorten",
    request_body = CreateShortLinkRequest,
    responses(
    (status = 200, description = "Short link created", body = ShortLink),
    (status = 400, description = "Bad request")
    )
)]
async fn create_short_link(
    State(service): State<SharedUrlShortener>,
    Json(payload): Json<CreateShortLinkRequest>,
) -> Result<Json<ShortLink>, String> {
    let mut service = service.lock().unwrap();

    let url = Url(payload.url);

    service
        .handle_create_short_link(url, None)
        .map(Json)
        .map_err(|e| format!("{e:?}"))
}


/// Create a short link with a predefined slug
#[utoipa::path(
    post,
    path = "/shorten/with-slug",
    request_body = CreateShortLinkWithSlugRequest,
    responses(
    (status = 200, description = "Short link created with predefined slug", body = ShortLink),
    (status = 400, description = "Slug already exists or bad request")
    )
)]
async fn create_short_link_with_slug(
    State(service): State<SharedUrlShortener>,
    Json(payload): Json<CreateShortLinkWithSlugRequest>,
) -> Result<Json<ShortLink>, String> {
    let mut service = service.lock().unwrap();

    let url = Url(payload.url);
    let slug = Slug(payload.slug);

    match service.handle_create_short_link(url, Some(slug)) {
        Ok(link) => Ok(Json(link)),
        Err(e) => Err(format!("Error: {e:?}")),
    }
}

/// Redirect by slug
#[utoipa::path(
    get,
    path = "/redirect/{slug}",
    responses(
        (status = 200, description = "Redirect handled", body = ShortLink),
        (status = 404, description = "Slug not found")
    )
)]
async fn redirect_by_slug(
    State(service): State<SharedUrlShortener>,
    Path(slug): Path<String>,
) -> Result<Json<ShortLink>, String> {
    let mut service = service.lock().unwrap();
    let slug = Slug(slug);

    service
        .handle_redirect(slug)
        .map(Json)
        .map_err(|e| format!("{e:?}"))
}

/// Get stats for a short link
#[utoipa::path(
    get,
    path = "/stats/{slug}",
    responses(
        (status = 200, description = "Statistics retrieved", body = StatsResponse),
        (status = 404, description = "Slug not found")
    )
)]
async fn fetch_stats(
    State(service): State<SharedUrlShortener>,
    Path(slug): Path<String>,
) -> Result<Json<StatsResponse>, String> {
    let service = service.lock().unwrap();
    let slug = Slug(slug);

    service.get_stats(slug).map(|stats| {
        Json(StatsResponse {
            slug: stats.link.slug.0,
            url: stats.link.url.0,
            redirects: stats.redirects,
        })
    }).map_err(|e| format!("{e:?}"))
}

/// `OpenAPI` documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        create_short_link,
        create_short_link_with_slug,
        redirect_by_slug,
        fetch_stats
    ),
    components(
        schemas(CreateShortLinkRequest, StatsResponse, ShortLink)
    ),
    tags(
        (name = "Url Shortener", description = "Operations for URL shortening service")
    )
)]
pub struct Doc;

/// Create the router for the application
pub fn create_router(service: SharedUrlShortener) -> Router {
    let api_router = Router::new()
        .route("/shorten", post(create_short_link))
        .route("/shorten/with-slug", post(create_short_link_with_slug))
        .route("/redirect/:slug", get(redirect_by_slug))
        .route("/stats/:slug", get(fetch_stats));

    Router::new()
        .merge(Scalar::with_url("/scalar", Doc::openapi()))
        .merge(api_router)
        .with_state(service)
}
