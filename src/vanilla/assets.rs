use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::Result;

pub const RESOURCES_URL: &str = "https://resources.download.minecraft.net/";

#[derive(Debug, Serialize, Deserialize)]
pub struct MCAssetIndex {
    pub map_to_resources: bool,
    pub objects: HashMap<String, MCAsset>,
}

impl Default for MCAssetIndex {
    fn default() -> Self {
        Self {
            map_to_resources: false,
            objects: HashMap::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCAsset {
    pub hash: String,
    pub size: u64,
}

impl MCAsset {
    pub async fn download(self, client: reqwest::Client) -> Result<reqwest::Response> {
        Ok(client.get(self.get_url()).send().await?)
    }

    /// get the url for downloading this asset
    pub fn get_url(self) -> String {
        RESOURCES_URL.to_owned() + &self.get_path()
    }

    /// get the path for this asset - no slashes at beginning or end
    pub fn get_path(self) -> String {
        self.hash[0..2].to_owned() + "/" + &self.hash
    }
}
