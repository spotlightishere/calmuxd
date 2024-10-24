use crate::FeedConfig;
use axum::{http::header, response::IntoResponse};

pub async fn handle_feed(feed: FeedConfig) -> impl IntoResponse {
    // TODO(spotlightishere): Implement feed combination
    let temporary_color = format!("Current color: {}", feed.color);

    ([(header::CONTENT_TYPE, "text/calendar")], temporary_color)
}
