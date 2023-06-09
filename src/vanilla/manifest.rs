use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct VersionManifest {
    pub latest: LatestVersions,
    pub versions: Vec<VersionIndex>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LatestVersions {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VersionIndex {
    id: String,
    #[serde(rename = "type")]
    version_type: VersionType,
    url: String,
    time: String,
    #[serde(rename = "releaseTime")]
    release_time: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum VersionType {
    Release,
    Snapshot,
}
