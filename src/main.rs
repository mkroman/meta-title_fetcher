#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;

use clap::{crate_authors, crate_version, App, Arg};

use std::fs::File;

mod api_v1;
mod config;
mod error;
mod url_handler;

use config::Config;
use error::Error;

fn main() -> Result<(), Error> {
    let matches = App::new("meta-title_fetcher")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("config")
                .help("Sets a custom config file")
                .short("c")
                .value_name("FILE")
                .takes_value(true),
        )
        .get_matches();

    let config = match matches.value_of("config") {
        Some(path) => match Config::read_from(&mut File::open(path)?) {
            Ok(config) => config,
            Err(err) => {
                println!("Error when loading config: {}", err);

                return Err(err);
            }
        },
        None => Config::default(),
    };

    let handler_registry = url_handler::init_registry();

    rocket::ignite()
        .manage(config)
        .manage(handler_registry)
        .mount("/v1/", routes![api_v1::fetch])
        .launch();

    Ok(())
}
