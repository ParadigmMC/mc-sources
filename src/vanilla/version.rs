use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::VersionType;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionInfo {
    pub id: String,
    pub assets: String,
    pub asset_index: PistonFile,
    pub java_version: VersionJavaInfo,
    pub libraries: Vec<PistonLibrary>,

    pub downloads: VersionDownloads,

    pub arguments: VersionArguments,
    pub minecraft_arguments: String,
    
    pub compliance_level: u8,
    pub minimum_launcher_version: u8,

    pub main_class: String,
    pub logging: LoggingInfoWrapper,

    #[serde(rename = "type")]
    pub version_type: VersionType,
    pub time: String,
    pub release_time: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct VersionDownloads {
    pub client: PistonFile,
    pub client_mappings: PistonFile,
    pub server: PistonFile,
    pub server_mappings: PistonFile,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionJavaInfo {
    pub major_version: u8,
    pub component: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VersionArguments {
    pub game: Vec<PistonArgument>,
    pub jvm: Vec<PistonArgument>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PistonArgument {
    Global(String),
    WithRules {
        rules: Vec<PistonRule>,
        value: MaybeVecArgument,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MaybeVecArgument {
    Vec(Vec<String>),
    NotVec(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PistonLibrary {
    pub name: String,
    pub downloads: PistonArtifact,
    pub rules: Vec<PistonRule>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "action")]
pub enum PistonRule {
    Allow(PistonRuleConstraints),
    Deny(PistonRuleConstraints),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PistonRuleConstraints {
    pub os: PistonRuleConstraintOS,
    pub features: HashMap<String, bool>,
}

/* 
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PistonRuleConstraintFeature {
    pub is_demo_user: bool,
    pub has_custom_resolution: bool,
    pub has_quick_plays_support: bool,

} */

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PistonRuleConstraintOS {
    pub name: String,
    pub arch: String,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PistonArtifact {
    pub artifact: PistonFile,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggingInfoWrapper {
    pub client: VersionLoggingInfo,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionLoggingInfo {
    pub argument: String,
    pub file: PistonFile,
    #[serde(rename = "type")]
    pub logging_type: String,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PistonFile {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    pub url: String,

    pub total_size: u64,
    
    pub path: String,
}
