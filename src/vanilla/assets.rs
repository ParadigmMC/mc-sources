use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::Result;

pub const RESOURCES_URL: &str = "https://resources.download.minecraft.net";

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MCAssetIndex {
    pub map_to_resources: bool,
    pub objects: HashMap<String, MCAsset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCAsset {
    pub hash: String,
    pub size: u64,
}

impl MCAsset {
    pub async fn download(&self, client: &reqwest::Client) -> Result<reqwest::Response> {
        Ok(client
            .get(self.get_url())
            .send()
            .await?
            .error_for_status()?)
    }

    /// get the url for downloading this asset
    #[must_use]
    pub fn get_url(&self) -> String {
        format!("{RESOURCES_URL}/{}", self.get_path())
    }

    /// get the path for this asset - no slashes at beginning or end
    #[must_use]
    pub fn get_path(&self) -> String {
        self.hash[0..2].to_owned() + "/" + &self.hash
    }
}
