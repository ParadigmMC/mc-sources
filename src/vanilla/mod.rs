use crate::Result;

mod manifest;
mod version;

pub use crate::vanilla::{manifest::*, version::*};


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

pub async fn fetch_vanilla(version: &str, client: &reqwest::Client) -> Result<reqwest::Response> {
    let version_manifest: VersionManifest = client
        .get("https://piston-meta.mojang.com/mc/game/version_manifest.json")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let mut target_version = version;

    if target_version == "latest" {
        target_version = &version_manifest.latest.release;
    }

    if target_version == "latest-snapshot" {
        target_version = &version_manifest.latest.snapshot;
    }

    let verdata = version_manifest
        .versions
        .iter()
        .find(|&v| v.id == target_version);

    let Some(verdata) = verdata else {
        bail!("Can't find the server jar for version {target_version}")
    };

    let package_manifest: PackageManifest = client
        .get(&verdata.url)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let res = client
        .get(package_manifest.downloads.server.url)
        .send()
        .await?
        .error_for_status()?;

    Ok(res)
}
