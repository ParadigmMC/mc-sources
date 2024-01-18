//! mcapi provides api's for various minecraft related projects such as:
//! - piston-meta, mojang's launcher api
//! - papermc
//! - modrinth
//! - spigot
//!
//! most functions use a `reqwest::Client` and are async

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod version;
pub use version::{MCVersion, MCVersionReq};
pub mod vanilla;

pub mod fabric;
pub mod forge;
pub mod hangar;
pub mod mclogs;
pub mod papermc;
pub mod purpurmc;
pub mod quilt;

/// Possible errors in this library
#[derive(Error, Debug)]
pub enum Error {
    #[error("{0} was not found")]
    NotFound(String),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error("{0} is an invalid MCVersion")]
    InvalidVersion(String),
    #[error(transparent)]
    Regex(#[from] regex::Error),
    #[error(transparent)]
    XML(#[from] roxmltree::Error),
}

type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Side {
    Server,
    Client,
}

lazy_static! {
    static ref DOLLAR_REGEX: Regex = Regex::new(r"\$\{(\w+)?\}").unwrap();
}

/// Utility fn for replacing strings containing "${}"
pub fn dollar_repl<F>(input: &str, replacer: F) -> String
where
    F: Fn(&str) -> Option<String>,
{
    DOLLAR_REGEX
        .replace_all(input, |caps: &regex::Captures| {
            let var_name = caps.get(1).map(|v| v.as_str()).unwrap_or_default();

            if let Some(v) = replacer(var_name) {
                v
            } else {
                format!("${{{var_name}}}")
            }
        })
        .into_owned()
}
