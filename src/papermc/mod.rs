//! API implementation for [PaperMC](https://papermc.io/)
//! This includes:
//! - Paper
//! - Folia
//! - Velocity (Proxy)
//! - Waterfall (Proxy)

use crate::Result;

const PAPERMC_URL: &str = "https://api.papermc.io/v2";

mod impls;
mod structs;
pub use structs::*;

/// Fetch a list of papermc projects (paper, folia, waterfall, velocity)
pub async fn fetch_papermc_projects(client: &reqwest::Client) -> Result<Vec<String>> {
    let projects: PaperProjectsResponse = client
        .get(PAPERMC_URL.to_owned() + "/projects")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(projects.projects)
}

/// Fetch versions of a project
pub async fn fetch_papermc_project(
    client: &reqwest::Client,
    project_id: &str,
) -> Result<PaperProject> {
    let project: PaperProject = client
        .get(format!("{PAPERMC_URL}/projects/{project_id}"))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(project)
}

pub async fn fetch_papermc_version(
    client: &reqwest::Client,
    project_id: &str,
    version: &str,
) -> Result<PaperVersion> {
    let version_response: PaperVersion = client
        .get(format!(
            "{PAPERMC_URL}/projects/{project_id}/versions/{version}"
        ))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(version_response)
}

pub async fn fetch_papermc_builds(
    client: &reqwest::Client,
    project_id: &str,
    version: &str,
) -> Result<PaperBuildsResponse> {
    let builds: PaperBuildsResponse = client
        .get(format!(
            "{PAPERMC_URL}/projects/{project_id}/versions/{version}/builds"
        ))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(builds)
}

pub async fn fetch_papermc_build(
    client: &reqwest::Client,
    project_id: &str,
    version: &str,
    build_id: i32,
) -> Result<PaperBuild> {
    let build: PaperBuild = client
        .get(format!(
            "{PAPERMC_URL}/projects/{project_id}/versions/{version}/builds/{build_id}"
        ))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(build)
}

pub async fn download_papermc_build(
    client: &reqwest::Client,
    project_id: &str,
    version: &str,
    build_id: i32,
    download_id: &str,
) -> Result<reqwest::Response> {
    Ok(client
        .get(
            format!("{PAPERMC_URL}/projects/{project_id}/versions/{version}/builds/{build_id}/downloads/{download_id}")
        )
        .send()
        .await?
        .error_for_status()?)
}

pub async fn fetch_papermc_version_group(
    client: &reqwest::Client,
    project_id: &str,
    family_id: &str,
) -> Result<PaperVersionFamily> {
    let family: PaperVersionFamily = client
        .get(format!(
            "{PAPERMC_URL}/projects/{project_id}/version_group/{family_id}"
        ))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(family)
}

pub async fn fetch_papermc_version_group_builds(
    client: &reqwest::Client,
    project_id: &str,
    family_id: &str,
) -> Result<PaperVersionFamilyBuildsResponse> {
    let builds: PaperVersionFamilyBuildsResponse = client
        .get(format!(
            "{PAPERMC_URL}/projects/{project_id}/version_group/{family_id}/builds"
        ))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(builds)
}
