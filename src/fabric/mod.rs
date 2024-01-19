use serde::{Deserialize, Serialize};

use crate::Result;

pub const FABRIC_META_URL: &str = "https://meta.fabricmc.net";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FabricVersion {
    pub version: String,
    pub stable: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FabricLoader {
    pub separator: String,
    pub build: i64,
    pub maven: String,
    pub version: String,
    pub stable: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FabricInstaller {
    pub url: String,
    pub maven: String,
    pub version: String,
    pub stable: bool,
}

pub async fn fetch_supported_versions(client: &reqwest::Client) -> Result<Vec<FabricVersion>> {
    Ok(client
        .get(FABRIC_META_URL.to_owned() + "/v2/versions/game")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?)
}

pub async fn fetch_loaders(client: &reqwest::Client) -> Result<Vec<FabricLoader>> {
    Ok(client
        .get(FABRIC_META_URL.to_owned() + "/v2/versions/loader")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?)
}

pub async fn fetch_installers(client: &reqwest::Client) -> Result<Vec<FabricInstaller>> {
    Ok(client
        .get(FABRIC_META_URL.to_owned() + "/v2/versions/installer")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?)
}

pub async fn download_server_jar(
    client: &reqwest::Client,
    game_version: &str,
    loader_version: &str,
    installer_version: &str,
) -> Result<reqwest::Response> {
    Ok(client
        .get(format!("{FABRIC_META_URL}/v2/versions/loader/{game_version}/{loader_version}/{installer_version}/server/jar"))
        .send()
        .await?
        .error_for_status()?)
}
