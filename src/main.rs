use axum::{routing::get, Router};
use config::{Config, FeedConfig};
use std::fs;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod muxer;

#[tokio::main]
async fn main() {
    // TODO(spotlightishere): Implement argument parsing
    // We should have our config location set to a defined path.
    let config_contents =
        fs::read_to_string("./config.json").expect("should be able to open configuration");
    let config: Config =
        serde_json::from_str(&config_contents).expect("should be able to parse configuration");

    // Enable tracing support. This can be useful for HTTP request logging.
    // Set the environmental variable `RUST_LOG` to `tower_http=debug`.
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Register a route for every configured feed.
    // We create a Router::new() and then create endpoints by their configured name.
    let routes = config
        .feeds
        .into_iter()
        .fold(Router::new(), |routes, feed| {
            let endpoint = feed.endpoint.clone();
            let method = get(move || muxer::handle_feed(feed));
            routes.route(&endpoint, method)
        });

    // Serve!
    let listener = tokio::net::TcpListener::bind(config.listen_address)
        .await
        .unwrap();
    axum::serve(listener, routes).await.unwrap();
}