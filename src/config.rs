use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    /// The address this server should listen on.
    /// Format is expected to match "host:port".
    pub listen_address: String,

    /// An array of `.ics` feeds to fetch and mux.
    pub feeds: Vec<FeedConfig>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FeedConfig {
    /// The endpoint, or path, to serve this feed under.
    /// For example, `http://localhost:8080/my-custom-feed`.
    pub endpoint: String,

    /// The name of the calendar to present for this feed.
    /// (This value is set within `X-WR-CALNAME`.)
    /// If not set, no name is provided.
    pub visual_name: Option<String>,

    /// The color that Apple devies will use for this feed.
    /// Expected form is hexadecimal: `#ff69bd`.
    pub color: String,

    /// The URLs to fetch and mux upon feed request.
    pub urls: Vec<String>,
}
