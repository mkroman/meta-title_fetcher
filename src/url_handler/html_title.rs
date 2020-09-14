use reqwest::redirect::Policy;
use std::time::Duration;

use crate::url_handler::{UrlHandler, UrlHandlerRegistry};

/// `HtmlTitleUrlHandler` is a basic handler that initiates a HTTP GET request and attempts to
/// extract a `<title>` element from the response body.
pub struct HtmlTitleUrlHandler {
    client: reqwest::blocking::Client,
}

impl HtmlTitleUrlHandler {
    /// Returns a HTML title handler.
    pub fn new() -> Self {
        let client = reqwest::blocking::Client::builder()
            .gzip(true)
            .timeout(Duration::from_secs(10))
            .redirect(Policy::limited(5))
            .build()
            .unwrap();

        HtmlTitleUrlHandler { client }
    }
}

impl UrlHandler for HtmlTitleUrlHandler {}

pub fn register(registry: &mut UrlHandlerRegistry) {
    registry.register(HtmlTitleUrlHandler::new());
}
