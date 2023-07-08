use serde::{Serialize, Deserialize};

use crate::Result;

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

pub static FABRIC_META_URL: &str = "https://meta.fabricmc.net";

pub async fn fetch_supported_versions(
    client: &reqwest::Client,
) -> Result<Vec<FabricVersion>> {
    let versions: Vec<FabricVersion> = client.get(FABRIC_META_URL.to_owned() + "/v2/versions/game")
        .send()
        .await?
        .json()
        .await?;

    Ok(versions)
}

pub async fn fetch_loaders(
    client: &reqwest::Client,
) -> Result<Vec<FabricLoader>> {
    let versions: Vec<FabricLoader> = client.get(FABRIC_META_URL.to_owned() + "/v2/versions/loader")
        .send()
        .await?
        .json()
        .await?;

    Ok(versions)
}

pub async fn fetch_installers(
    client: &reqwest::Client,
) -> Result<Vec<FabricInstaller>> {
    let installers: Vec<FabricInstaller> = client.get(FABRIC_META_URL.to_owned() + "/v2/versions/installer")
        .send()
        .await?
        .json()
        .await?;

    Ok(installers)
}

pub async fn download_server_jar(
    client: &reqwest::Client,
    game_version: &str,
    loader_version: &str,
    installer_version: &str,
) -> Result<reqwest::Response> {
    Ok(
        client.get(FABRIC_META_URL.to_owned() + "/v2/versions/loader/"
            + game_version + "/"
            + loader_version + "/"
            + installer_version
            + "/server/jar")
        .send().await?
    )
}
