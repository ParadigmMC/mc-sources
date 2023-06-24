//! todo!() - broken because quilt, unlike fabric, needs libraries etc to be downloaded to run. ffuuuu

use serde::{Serialize, Deserialize};

use crate::Result;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QuiltVersion {
    pub version: String,
    pub stable: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QuiltLoader {
    pub seperator: String,
    pub build: i64,
    pub maven: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QuiltInstaller {
    pub url: String,
    pub maven: String,
    pub version: String,
}

pub static QUILT_META_URL: &str = "https://meta.quiltmc.org";
pub static QUILT_MAVEN_URL: &str = "https://maven.quiltmc.org";

pub async fn fetch_supported_versions(
    client: &reqwest::Client,
) -> Result<Vec<QuiltVersion>> {
    let versions: Vec<QuiltVersion> = client.get(QUILT_META_URL.to_owned() + "/v3/versions/game")
        .send()
        .await?
        .json()
        .await?;

    Ok(versions)
}

pub async fn fetch_loaders(
    client: &reqwest::Client,
) -> Result<Vec<QuiltLoader>> {
    let versions: Vec<QuiltLoader> = client.get(QUILT_META_URL.to_owned() + "/v3/versions/loader")
        .send()
        .await?
        .json()
        .await?;

    Ok(versions)
}

pub async fn fetch_installers(
    client: &reqwest::Client,
) -> Result<Vec<QuiltInstaller>> {
    let installers: Vec<QuiltInstaller> = client.get(QUILT_META_URL.to_owned() + "/v3/versions/installer")
        .send()
        .await?
        .json()
        .await?;

    Ok(installers)
}

pub async fn download_installer_jar(
    client: &reqwest::Client,
    installer_version: &str,
) -> Result<reqwest::Response> {
    Ok(
        client.get(QUILT_MAVEN_URL.to_owned() + 
            "/repository/release/org/quiltmc/quilt-installer/"
            + installer_version +
            "/quilt-installer-"
            + installer_version +
            ".jar")
        .send().await?
    )
}
