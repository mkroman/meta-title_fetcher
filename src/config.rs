use serde::Deserialize;

use std::io::Read;
use std::path::Path;

use crate::Error;

/// The default User-Agent header value.
const DEFAULT_USER_AGENT: &'static str =
    "Mozilla/5.0 (X11; Linux x86_64; rv:71.0) Gecko/20100101 Firefox/71.0";

/// The default setting for the `max_content_length` config option.
/// Defaults to 4 MiB.
const DEFAULT_MAX_CONTENT_LENGTH: u64 = (4 * 1024 * 1024);

/// The default value for maximum number of redirects.
const DEFAULT_MAX_REDIRECTS: u64 = 5;

/// The default setting for the `timeout` config option, in seconds.
const DEFAULT_TIMEOUT: u64 = 10;

#[derive(Debug, Deserialize)]
pub struct HttpConfig {
    /// The number of bytes to process in a request before closing the connection.
    pub max_content_length: u64,
    /// The User-Agent header sent with each request.
    pub user_agent: String,
    /// The maximum number of redirects in a single request.
    pub max_redirects: u64,
    /// The time in seconds before a request is to be considered timed out.
    pub timeout: u64,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub http: HttpConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            http: HttpConfig {
                max_content_length: DEFAULT_MAX_CONTENT_LENGTH,
                user_agent: DEFAULT_USER_AGENT.to_string(),
                max_redirects: DEFAULT_MAX_REDIRECTS,
                timeout: DEFAULT_TIMEOUT,
            },
        }
    }
}

impl Config {
    /// Returns a new Config with default values.
    pub fn new() -> Config {
        Default::default()
    }

    pub fn read_from<R: Read>(reader: &mut R) -> Result<Config, Error> {
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;

        let config = toml::from_str(&buffer).map_err(|e| e.into());

        return config;
    }
}
