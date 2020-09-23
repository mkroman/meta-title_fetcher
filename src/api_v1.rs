use lazy_static::lazy_static;
use rocket::http::RawStr;
use rocket::request::{Form, FromForm, FromFormValue};
use rocket::{post, routes, Rocket, State};
use rocket_contrib::json::Json;
use scraper::{Html, Selector};
use serde::Serialize;
use url::Url;

use std::io::Read;
use std::time::Duration;

use reqwest::redirect::Policy;

use crate::{Config, Error};

struct Uri(Url);

impl From<Url> for Uri {
    fn from(url: Url) -> Self {
        Uri(url)
    }
}

impl Uri {
    pub fn as_url(&self) -> &Url {
        &self.0
    }
}

impl<'v> FromFormValue<'v> for Uri {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<Uri, &'v RawStr> {
        match form_value.parse::<Url>() {
            Ok(url) => Ok(url.into()),
            _ => Err(form_value),
        }
    }
}

#[derive(FromForm)]
pub struct UserInput {
    uri: Option<Uri>,
}

#[derive(Serialize)]
pub struct Document {
    pub title: String,
    pub bytes_read: usize,
}

lazy_static! {
    static ref CLIENT: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .gzip(true)
        .timeout(Duration::from_secs(10))
        .redirect(Policy::limited(5))
        .build()
        .unwrap();
}

pub fn get_title(url: &Url, config: &Config) -> Result<Option<Document>, Error> {
    use reqwest::header::USER_AGENT;

    let response = CLIENT
        .get(url.as_str())
        .header(USER_AGENT, &config.http.user_agent)
        .send()?;

    let _content_length = match response.content_length() {
        Some(length) => {
            if length < config.http.max_content_length {
                length
            } else {
                return Err(Error::ContentTooBigError(length));
            }
        }
        None => 0,
    };

    let mut buffer = String::with_capacity(config.http.max_content_length as usize);

    let num_read = response
        .take(config.http.max_content_length)
        .read_to_string(&mut buffer)?;

    let document = Html::parse_document(buffer.as_str());
    let selector = Selector::parse("title").unwrap();
    let title = document
        .select(&selector)
        .next()
        .ok_or(Error::NoValidTitleError)
        .map(|e| e.text().collect::<Vec<_>>())?
        .join(" ");

    Ok(Some(Document {
        title,
        bytes_read: num_read,
    }))
}

#[post("/fetch", data = "<user_input>")]
pub fn fetch(user_input: Form<UserInput>, config: State<Config>) -> Result<Json<Document>, Error> {
    let url = match &user_input.uri {
        Some(uri) => uri.as_url(),
        None => return Err(Error::UriParseError),
    };

    let document = get_title(url, &config.inner())?;
    document.ok_or(Error::NoValidTitleError).map(Json)
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/v1/", routes![fetch])
}
