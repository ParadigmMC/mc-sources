use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod vanilla;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0} was not found")]
    NotFound(String),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
}

type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Side {
    Server,
    Client,
}

