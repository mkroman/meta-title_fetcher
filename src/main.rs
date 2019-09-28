#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;

use rocket::http::RawStr;
use rocket::request::{Form, FromFormValue};
use rocket_contrib::json::Json;
use scraper::{Html, Selector};
use serde::Serialize;
use url::Url;

use std::io::Read;
use std::time::Duration;

use reqwest::header::USER_AGENT;
use reqwest::RedirectPolicy;

const MAX_CONTENT_LENGTH: u64 = (4 * 1024 * 1024);
const META_USER_AGENT: &'static str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:68.0) Gecko/20100101 Firefox/68.0";

mod error;
use error::Error;

mod url_handler;

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
struct UserInput {
    uri: Option<Uri>,
}

#[derive(Serialize)]
struct Document {
    pub title: String,
    pub bytes_read: usize,
}

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::builder()
        .gzip(true)
        .timeout(Duration::from_secs(10))
        .redirect(RedirectPolicy::limited(5))
        .build()
        .unwrap();
}

fn get_title(url: &Url) -> Result<Option<Document>, Error> {
    let response = CLIENT
        .get(url.as_str())
        .header(USER_AGENT, META_USER_AGENT)
        .send()?;

    let _content_length = match response.content_length() {
        Some(length) => {
            if length < MAX_CONTENT_LENGTH {
                length
            } else {
                return Err(Error::ContentTooBigError(length));
            }
        }
        None => 0,
    };

    let mut buffer = String::with_capacity(MAX_CONTENT_LENGTH as usize);

    let num_read = response
        .take(MAX_CONTENT_LENGTH)
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
        title: title.to_string(),
        bytes_read: num_read,
    }))
}

#[post("/fetch", data = "<user_input>")]
fn fetch(user_input: Form<UserInput>) -> Result<Json<Document>, Error> {
    let url = match &user_input.uri {
        Some(uri) => uri.as_url(),
        None => return Err(Error::UriParseError),
    };

    let document = get_title(url)?;
    document.ok_or(Error::NoValidTitleError).map(|d| Json(d))
}

fn main() {
    url_handler::init();

    rocket::ignite().mount("/", routes![fetch]).launch();
}
