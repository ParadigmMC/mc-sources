// ...todo!() - broken because quilt, unlike fabric, needs libraries etc to be downloaded to run. ffuuuu

use serde::{Deserialize, Serialize};

use crate::Result;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QuiltVersion {
    pub version: String,
    pub stable: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QuiltLoader {
    pub separator: String,
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

// https://github.com/QuiltMC/quiltmc.org/blob/main/functions/api/v1/download-latest-installer/%5Barch%5D.js

pub const QUILT_META_URL: &str = "https://meta.quiltmc.org";
pub const QUILT_MAVEN_URL: &str = "https://maven.quiltmc.org";
pub const QUILT_INSTALLER_UNIVERSAL_PATH: &str = "/repository/release/org/quiltmc/quilt-installer";
pub const QUILT_INSTALLER_NATIVE_PATH: &str =
    "/repository/release/org/quiltmc/quilt-installer-native-bootstrap";
pub const METADATA: &str = "maven-metadata.xml";
pub const VERSION_REGEX: &str = "<version>(.+?)</version>";

pub enum InstallerVariant {
    Universal,
    Native(String),
}

impl InstallerVariant {
    #[must_use]
    pub fn get_metadata_url(&self) -> String {
        QUILT_MAVEN_URL.to_owned()
            + &(match self {
                Self::Universal => QUILT_INSTALLER_UNIVERSAL_PATH.to_owned(),
                Self::Native(arch) => QUILT_INSTALLER_NATIVE_PATH.to_owned() + arch + "/",
            })
            + METADATA
    }

    #[must_use]
    pub fn get_artifact_url(&self, version: &str) -> String {
        match self {
            Self::Universal => format!("{QUILT_MAVEN_URL}{QUILT_INSTALLER_UNIVERSAL_PATH}/{version}/quilt-installer-{version}.jar"),
            Self::Native(arch) => {
                let ext = if arch.starts_with("windows") {
                    ".exe"
                } else {
                    ""
                };

                format!("{QUILT_MAVEN_URL}{QUILT_INSTALLER_NATIVE_PATH}/{arch}/{version}/{arch}-{version}{ext}")
            }
        }
    }

    pub async fn fetch_versions(&self, client: &reqwest::Client) -> Result<Vec<String>> {
        fetch_installer_versions(client, self).await
    }
}

pub async fn fetch_installer_versions(
    client: &reqwest::Client,
    variant: &InstallerVariant,
) -> Result<Vec<String>> {
    let xml = client
        .get(variant.get_metadata_url())
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    Ok(roxmltree::Document::parse(&xml)?
        .descendants()
        .filter_map(|t| {
            if t.tag_name().name() == "version" {
                Some(t.text()?.to_owned())
            } else {
                None
            }
        })
        .collect())
}

pub async fn download_installer(
    client: &reqwest::Client,
    variant: &InstallerVariant,
    version: &str,
) -> Result<reqwest::Response> {
    Ok(client
        .get(variant.get_artifact_url(version))
        .send()
        .await?
        .error_for_status()?)
}

pub async fn fetch_supported_versions(client: &reqwest::Client) -> Result<Vec<QuiltVersion>> {
    let versions: Vec<QuiltVersion> = client
        .get(QUILT_META_URL.to_owned() + "/v3/versions/game")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(versions)
}

pub async fn fetch_loaders(client: &reqwest::Client) -> Result<Vec<QuiltLoader>> {
    let versions: Vec<QuiltLoader> = client
        .get(QUILT_META_URL.to_owned() + "/v3/versions/loader")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(versions)
}

pub async fn fetch_installers(client: &reqwest::Client) -> Result<Vec<QuiltInstaller>> {
    let installers: Vec<QuiltInstaller> = client
        .get(QUILT_META_URL.to_owned() + "/v3/versions/installer")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(installers)
}
