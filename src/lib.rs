use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod version;
pub use version::{MCVersion, MCVersionReq};
pub mod vanilla;

pub mod papermc;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0} was not found")]
    NotFound(String),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error("{0} is an invalid MCVersion")]
    InvalidVersion(String),
}

type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Side {
    Server,
    Client,
}

