use axum::extract::{Path, State};
use axum::{Json, Router};
use axum::routing::{get, post};
use crate::commands::CommandHandler;
use crate::queries::QueryHandler;
use crate::service::SharedService;
use crate::structs::{ShortLink, Slug, Url};

/// Payload for creating a short link
#[derive(serde::Deserialize)]
struct CreateShortLinkRequest {
    url: String,
    slug: Option<String>,
}

/// Response for stats
#[derive(serde::Serialize)]
struct StatsResponse {
    slug: String,
    url: String,
    redirects: u64,
}

/// Create a short link
#[axum::debug_handler]
async fn create_short_link(
    State(service): State<SharedService>,
    Json(payload): Json<CreateShortLinkRequest>,
) -> Result<Json<ShortLink>, String> {
    let mut service = service.lock().unwrap();

    let url = Url(payload.url);
    let slug = payload.slug.map(Slug);

    service
        .handle_create_short_link(url, slug)
        .map(Json)
        .map_err(|e| format!("{:?}", e))
}

/// Redirect by slug.
async fn handle_redirect(
    State(service): State<SharedService>,
    Path(slug): Path<String>,
) -> Result<Json<ShortLink>, String> {
    let mut service = service.lock().unwrap();
    let slug = Slug(slug);

    service
        .handle_redirect(slug)
        .map(Json)
        .map_err(|e| format!("{:?}", e))
}

/// Get stats for a short link
async fn get_stats(
    State(service): State<SharedService>,
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
    }).map_err(|e| format!("{:?}", e))
}

pub fn create_router(service: SharedService) -> Router {
    Router::new()
        .route("/shorten", post(create_short_link))
        .route("/redirect/:slug", get(handle_redirect))
        .route("/stats/:slug", get(get_stats))
        .with_state(service)
}
