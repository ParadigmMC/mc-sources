use std::collections::HashMap;

use crate::Result;

pub const FORGE_MANIFEST_URL: &str =
    "https://files.minecraftforge.net/net/minecraftforge/forge/maven-metadata.json";

pub async fn fetch_versions(client: &reqwest::Client) -> Result<HashMap<String, Vec<String>>> {
    Ok(client
        .get(FORGE_MANIFEST_URL)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?)
}
