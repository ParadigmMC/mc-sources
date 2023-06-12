use super::*;

impl PaperProject {
    pub async fn fetch_version(self, client: &reqwest::Client, version: &str) -> Result<PaperVersion> {
        Ok(fetch_papermc_version(client, &self.project_id, version).await?)
    }

    pub async fn fetch_version_group(self, client: &reqwest::Client, version_group: &str) -> Result<PaperVersionFamily> {
        Ok(fetch_papermc_version_group(client, &self.project_id, version_group).await?)
    }
}

impl PaperVersion {
    pub async fn fetch_build(self, client: &reqwest::Client, build: i32) -> Result<PaperBuild> {
        Ok(fetch_papermc_build(client, &self.project_id, &self.version, build).await?)
    }
}

impl PaperBuild {
    pub async fn download(self, client: &reqwest::Client, file: &str) -> Result<reqwest::Response> {
        Ok(download_papermc_build(client, &self.project_id, &self.version, self.build, file).await?)
    }
}
