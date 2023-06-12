use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PaperVersion {
    pub project_id: String,
    pub project_name: String,
    pub version: String,
    pub builds: Vec<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaperBuild {
    pub project_id: String,
    pub project_name: String,
    pub version: String,
    pub build: i32,
    pub time: String,
    pub channel: PaperChannel,
    pub promoted: bool,
    pub changes: Vec<PaperChange>,
    pub downloads: HashMap<String, PaperDownload>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PaperChannel {
    Default,
    Experimental,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaperChange {
    pub commit: String,
    pub summary: String,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaperBuildsResponse {
    pub project_id: String,
    pub project_name: String,
    pub version: String,
    pub builds: Vec<PaperVersionBuild>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaperDownload {
    pub name: String,
    pub sha256: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaperProject {
    pub project_id: String,
    pub project_name: String,
    pub version_groups: Vec<String>,
    pub versions: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaperProjectsResponse {
    pub projects: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaperVersionBuild {
    pub build: i32,
    pub time: String,
    pub channel: PaperChannel,
    pub promoted: bool,
    pub changes: Vec<PaperChange>,
    pub downloads: HashMap<String, PaperDownload>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaperVersionFamily {
    pub project_id: String,
    pub project_name: String,
    pub version_group: String,
    pub versions: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaperVersionFamilyBuildsResponse {
    pub project_id: String,
    pub project_name: String,
    pub version_group: String,
    pub versions: Vec<String>,
    pub builds: Vec<PaperVersionFamilyBuild>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaperVersionFamilyBuild {
    pub version: String,
    pub build: i32,
    pub time: String,
    pub channel: PaperChannel,
    pub promoted: bool,
    pub changes: Vec<PaperChange>,
    pub downloads: HashMap<String, PaperDownload>,
}

