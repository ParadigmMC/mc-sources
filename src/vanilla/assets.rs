use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct MCAssets {
    map_to_resources: bool,
    objects: HashMap<String, MCAsset>,
}

impl Default for MCAssets {
    fn default() -> Self {
        Self {
            map_to_resources: false,
            objects: HashMap::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCAsset {
    hash: String,
    size: u64,
}

impl MCAsset {
    pub async fn download(self, client: reqwest::Client) -> Result<reqwest::Response> {
        Ok(client.get("https://resources.download.minecraft.net/".to_owned()
            + &self.hash[0..2] + "/" + &self.hash).send().await?)
    }
}
