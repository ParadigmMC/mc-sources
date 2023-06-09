use crate::{Result, Error};

mod manifest;
mod version;
mod assets;

pub use crate::vanilla::{manifest::*, version::*, assets::*};

pub async fn fetch_version_manifest(
    client: &reqwest::Client,
) -> Result<VersionManifest> {
    let version_manifest: VersionManifest = client
        .get("https://piston-meta.mojang.com/mc/game/version_manifest.json")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(version_manifest)
}

impl VersionManifest {
    pub fn find(self, id: &str) -> Option<VersionIndex> {
        self.versions
            .iter()
            .find(|v| v.id == id).map(|v| v.clone())
    }

    pub async fn fetch_latest_release(self, client: &reqwest::Client) -> Result<VersionInfo> {
        let id = self.latest.release.clone();
        self.fetch(&id, client).await
    }
    
    pub async fn fetch_latest_snapshot(self, client: &reqwest::Client) -> Result<VersionInfo> {
        let id = self.latest.snapshot.clone();
        self.fetch(&id, client).await
    }

    pub async fn fetch(self, id: &str, client: &reqwest::Client) -> Result<VersionInfo> {
        self
            .find(id)
            .ok_or(Error::NotFound(id.to_owned()))?.fetch(client).await
    }
}

impl VersionIndex {
    pub async fn fetch(self, client: &reqwest::Client) -> Result<VersionInfo> {
        Ok(client.get(self.url).send().await?.error_for_status()?.json().await?)
    }
}

impl PistonLibrary {
    pub async fn download(self, client: &reqwest::Client) -> Result<reqwest::Response> {
        self.downloads.download(client).await
    }
}

impl PistonArtifact {
    pub async fn download(self, client: &reqwest::Client) -> Result<reqwest::Response> {
        self.artifact.download(client).await
    }
}

impl PistonFile {
    pub async fn download(self, client: &reqwest::Client) -> Result<reqwest::Response> {
        Ok(client.get(self.url).send().await?.error_for_status()?)
    }
}
