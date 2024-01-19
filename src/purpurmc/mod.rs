//! API implementation for [PurpurMC](https://purpurmc.org/)

use serde::{Deserialize, Serialize};

use crate::{Error, Result};

pub const PURPURMC_URL: &str = "https://api.purpurmc.org/v2";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PurpurVersion {
    pub project: String,
    pub version: String,
    pub builds: PurpurBuilds<PurpurBuild>,
}

impl PurpurVersion {
    #[must_use]
    pub fn get_latest_build(&self) -> PurpurBuild {
        self.builds.latest.clone()
    }

    #[must_use]
    pub fn get_build(&self, build_id: &str) -> Option<&PurpurBuild> {
        self.builds.all.iter().find(|b| b.build == build_id)
    }

    pub async fn download_latest_build(
        &self,
        client: &reqwest::Client,
    ) -> Result<reqwest::Response> {
        self.get_latest_build().download(client).await
    }

    pub async fn download_build(
        &self,
        client: &reqwest::Client,
        build_id: &str,
    ) -> Result<reqwest::Response> {
        self.get_build(build_id)
            .ok_or_else(|| {
                Error::NotFound(format!("PurpurMC ver: {} build: {build_id}", self.version))
            })?
            .download(client)
            .await
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PurpurVersionShort {
    pub project: String,
    pub version: String,
    pub builds: PurpurBuilds<String>,
}

impl PurpurVersionShort {
    #[must_use]
    pub fn get_latest_build_id(&self) -> String {
        self.builds.latest.clone()
    }

    pub async fn fetch_latest_build(&self, client: &reqwest::Client) -> Result<PurpurBuild> {
        fetch_purpur_build(client, &self.version, &self.get_latest_build_id()).await
    }

    pub async fn fetch_build(
        &self,
        client: &reqwest::Client,
        build_id: &str,
    ) -> Result<PurpurBuild> {
        fetch_purpur_build(client, &self.version, build_id).await
    }

    pub async fn download_latest_build(
        &self,
        client: &reqwest::Client,
    ) -> Result<reqwest::Response> {
        self.fetch_latest_build(client)
            .await?
            .download(client)
            .await
    }

    pub async fn download_build(
        &self,
        client: &reqwest::Client,
        build_id: &str,
    ) -> Result<reqwest::Response> {
        self.fetch_build(client, build_id)
            .await?
            .download(client)
            .await
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PurpurBuilds<T> {
    pub latest: T,
    pub all: Vec<T>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PurpurBuild {
    pub project: String,
    pub version: String,
    pub build: String,
    pub result: String,
    pub timestamp: i64,
    pub duration: i64,
    pub commits: Vec<PurpurCommit>,
    pub md5: String,
}

impl PurpurBuild {
    pub async fn download(&self, client: &reqwest::Client) -> Result<reqwest::Response> {
        download_purpur_build(client, &self.version, &self.build).await
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PurpurCommit {
    pub author: String,
    pub email: String,
    pub description: String,
    pub hash: String,
    pub timestamp: i64,
}

/// Fetch a list of purpurmc versions
pub async fn fetch_purpur_versions(client: &reqwest::Client) -> Result<Vec<String>> {
    Ok(client
        .get(PURPURMC_URL.to_owned() + "/purpur")
        .send()
        .await?
        .error_for_status()?
        .json::<Vec<String>>()
        .await?)
}

/// Fetch the builds of a Purpur version
/// Use [`fetch_purpur_version_short()`] to get only the id's of the builds
pub async fn fetch_purpur_version(
    client: &reqwest::Client,
    version: &str,
) -> Result<PurpurVersion> {
    Ok(client
        .get(format!("{PURPURMC_URL}/purpur/{version}?detailed=true"))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?)
}

/// Use [`fetch_purpur_version()`] for a more detailed response
/// Returns the build id's in the response instead of `PurpurBuild` structs
pub async fn fetch_purpur_version_short(
    client: &reqwest::Client,
    version: &str,
) -> Result<PurpurVersionShort> {
    Ok(client
        .get(format!("{PURPURMC_URL}/purpur/{version}"))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?)
}

pub async fn fetch_purpur_build(
    client: &reqwest::Client,
    version: &str,
    build_id: &str,
) -> Result<PurpurBuild> {
    Ok(client
        .get(format!("{PURPURMC_URL}/purpur/{version}/{build_id}"))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?)
}

pub async fn download_purpur_build(
    client: &reqwest::Client,
    version: &str,
    build_id: &str,
) -> Result<reqwest::Response> {
    Ok(client
        .get(format!(
            "{PURPURMC_URL}/purpur/{version}/{build_id}/download"
        ))
        .send()
        .await?
        .error_for_status()?)
}
