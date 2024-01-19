use std::collections::HashMap;

use reqwest::header::{HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use thiserror::Error;

const API_V1: &str = "https://api.mclo.gs/1";

#[derive(Error, Debug)]
pub enum MCLogsError {
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    JSON(#[from] serde_json::Error),
    #[error("{0}")]
    APIError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogFileMetadata {
    pub id: String,
    pub url: String,
    pub raw: String,
}

impl LogFileMetadata {
    pub async fn fetch_raw(&self, http_client: &reqwest::Client) -> Result<String, MCLogsError> {
        fetch_raw_log(http_client, &self.id).await
    }

    pub async fn fetch_insights(
        &self,
        http_client: &reqwest::Client,
    ) -> Result<LogInsights, MCLogsError> {
        fetch_insights(http_client, &self.id).await
    }
}

pub async fn post_log(
    http_client: &reqwest::Client,
    content: &str,
) -> Result<LogFileMetadata, MCLogsError> {
    let params = HashMap::from([("content", content)]);

    let json = http_client
        .post(format!("{API_V1}/log"))
        .form(&params)
        .send()
        .await?
        .error_for_status()?
        .json::<serde_json::Value>()
        .await?;

    let Some(b) = json.get("success") else {
        return Err(MCLogsError::APIError(
            "'success' field not in response".to_owned(),
        ));
    };

    match b.as_bool() {
        Some(true) => Ok(serde_json::from_value(json)?),
        Some(false) => Err(MCLogsError::APIError(
            json.get("error")
                .and_then(|e| e.as_str())
                .unwrap_or("unknown error")
                .to_owned(),
        )),
        None => Err(MCLogsError::APIError(
            "'success' field in response was not a bool".to_owned(),
        )),
    }
}

pub async fn fetch_raw_log(http_client: &reqwest::Client, id: &str) -> Result<String, MCLogsError> {
    let response = http_client
        .get(format!("{API_V1}/raw/{id}"))
        .send()
        .await?
        .error_for_status()?;

    let content_type = response.headers().get(CONTENT_TYPE).cloned();

    let text = response.text().await?;

    if content_type == Some(HeaderValue::from_static("application/json")) {
        let json = serde_json::from_str::<serde_json::Value>(&text)?;
        let message = json
            .get("error")
            .and_then(|e| e.as_str())
            .unwrap_or("unknown error");

        return Err(MCLogsError::APIError(message.to_owned()));
    }

    Ok(text)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogInsights {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub log_type: String,
    pub version: String,
    pub title: String,
    pub analysis: LogAnalysis,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogAnalysis {
    pub problems: Vec<Problem>,
    pub information: Vec<Information>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Problem {
    pub message: String,
    pub counter: usize,
    pub entry: AnalysisEntry,
    pub solutions: Vec<Solution>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Information {
    pub message: String,
    pub counter: usize,
    pub label: String,
    pub value: String,
    pub entry: AnalysisEntry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisEntry {
    pub level: usize,
    pub time: Option<String>,
    pub prefix: String,
    pub lines: Vec<AnalysisLine>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisLine {
    pub number: usize,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Solution {
    pub message: String,
}

pub async fn fetch_insights(
    http_client: &reqwest::Client,
    id: &str,
) -> Result<LogInsights, MCLogsError> {
    let json = http_client
        .get(format!("{API_V1}/insights/{id}"))
        .send()
        .await?
        .error_for_status()?
        .json::<serde_json::Value>()
        .await?;

    match json.get("success").map(|b| b.as_bool().unwrap_or(true)) {
        Some(true) | None => Ok(serde_json::from_value(json)?),
        Some(false) => Err(MCLogsError::APIError(
            json.get("error")
                .and_then(|e| e.as_str())
                .unwrap_or("unknown error")
                .to_owned(),
        )),
    }
}
