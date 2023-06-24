use serde::{Deserialize, Serialize};

/// The version manifest, from piston-meta
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VersionManifest {
    pub latest: LatestVersions,
    pub versions: Vec<VersionIndex>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LatestVersions {
    pub release: String,
    pub snapshot: String,
}

/// The version info from a manifest's versions list
/// Use [`Self::fetch()`] to get an [`VersionInfo`] which contains more info about the version
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionIndex {
    pub id: String,
    #[serde(rename = "type")]
    pub version_type: VersionType,
    pub url: String,
    pub time: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
    pub sha1: String,
    pub compliance_level: u8,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VersionType {
    Release,
    Snapshot,
    OldAlpha,
    OldBeta,
}
