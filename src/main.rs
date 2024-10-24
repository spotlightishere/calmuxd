use axum::{http::header, response::IntoResponse, routing::get, Router};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

struct Config {
    /// The address this server should listen on.
    /// Format is expected to match "host:port".
    listen_address: String,
}

#[tokio::main]
async fn main() {
    // TODO(spotlightishere): Implement configuration
    let config = Config {
        listen_address: "127.0.0.1:8080".to_string(),
    };

    // Enable tracing support. This can be useful for HTTP request logging.
    // Set the environmental variable `RUST_LOG` to `tower_http=debug`.
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // TODO(spotlightishere): Have individual routes per muxed feeds
    let app = Router::new()
        .route("/", get(root))
        .layer(TraceLayer::new_for_http());

    // Serve!
    let listener = tokio::net::TcpListener::bind(config.listen_address)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/calendar")], "foo")
}
