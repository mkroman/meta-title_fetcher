use serde::Deserialize;

use std::io::Read;

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
    #[serde(default = "default_max_content_length")]
    pub max_content_length: u64,

    /// The User-Agent header sent with each request.
    #[serde(default = "default_user_agent")]
    pub user_agent: String,

    /// The maximum number of redirects in a single request.
    #[serde(default = "default_max_redirects")]
    pub max_redirects: u64,

    /// The time in seconds before a request is to be considered timed out.
    #[serde(default = "default_timeout")]
    pub timeout: u64,
}

fn default_max_content_length() -> u64 {
    DEFAULT_MAX_CONTENT_LENGTH
}

fn default_user_agent() -> String {
    DEFAULT_USER_AGENT.to_string()
}

fn default_timeout() -> u64 {
    DEFAULT_TIMEOUT
}

fn default_max_redirects() -> u64 {
    DEFAULT_MAX_REDIRECTS
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_http_config")]
    pub http: HttpConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            http: HttpConfig::default(),
        }
    }
}

fn default_http_config() -> HttpConfig {
    HttpConfig::default()
}

impl Default for HttpConfig {
    fn default() -> Self {
        HttpConfig {
            max_content_length: DEFAULT_MAX_CONTENT_LENGTH,
            user_agent: DEFAULT_USER_AGENT.to_string(),
            max_redirects: DEFAULT_MAX_REDIRECTS,
            timeout: DEFAULT_TIMEOUT,
        }
    }
}

impl Config {
    /// Returns a Config struct that from a reader that has access to a toml config.
    pub fn read_from<R: Read>(reader: &mut R) -> Result<Config, Error> {
        let mut buffer = String::new();

        reader.read_to_string(&mut buffer)?;

        toml::from_str(&buffer).map_err(|e| e.into())
    }
}
