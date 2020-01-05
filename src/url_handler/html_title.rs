use reqwest::RedirectPolicy;
use std::time::Duration;

use crate::url_handler::{UrlHandler, UrlHandlerRegistry};

pub struct HtmlTitleUrlHandler {
    client: reqwest::Client,
}

impl HtmlTitleUrlHandler {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .gzip(true)
            .timeout(Duration::from_secs(10))
            .redirect(RedirectPolicy::limited(5))
            .build()
            .unwrap();

        HtmlTitleUrlHandler { client }
    }
}

impl UrlHandler for HtmlTitleUrlHandler {}

pub fn register(registry: &mut UrlHandlerRegistry) {
    registry.register(HtmlTitleUrlHandler::new());
}
