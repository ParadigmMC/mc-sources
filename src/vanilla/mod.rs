//! API implementation of piston-meta (mojang's launcher api)
//! Contains implementations for fetching versions, downloading, libraries and assets

use std::{collections::HashMap, env};

use regex::Regex;

use crate::{dollar_repl, Error, Result};

mod assets;
mod manifest;
mod version;

pub use crate::vanilla::{assets::*, manifest::*, version::*};

pub const VERSION_MANIFEST_URL: &str =
    "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

/// Fetches the version manifest
pub async fn fetch_version_manifest(client: &reqwest::Client) -> Result<VersionManifest> {
    let version_manifest: VersionManifest = client
        .get(VERSION_MANIFEST_URL)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(version_manifest)
}

impl VersionManifest {
    /// Find the version with id from the list
    #[must_use]
    pub fn find(&self, id: &str) -> Option<VersionIndex> {
        self.versions.iter().find(|v| v.id == id).cloned()
    }

    /// Fetch the latest release's `VersionInfo`
    pub async fn fetch_latest_release(&self, client: &reqwest::Client) -> Result<VersionInfo> {
        let id = self.latest.release.clone();
        self.fetch(&id, client).await
    }

    /// Fetch the latest snapshot's `VersionInfo`
    pub async fn fetch_latest_snapshot(&self, client: &reqwest::Client) -> Result<VersionInfo> {
        let id = self.latest.snapshot.clone();
        self.fetch(&id, client).await
    }

    /// Fetch the `VersionInfo` of id
    pub async fn fetch(&self, id: &str, client: &reqwest::Client) -> Result<VersionInfo> {
        self.find(id)
            .ok_or(Error::NotFound(id.to_owned()))?
            .fetch(client)
            .await
    }
}

impl VersionIndex {
    /// Fetch the `VersionInfo` from the manifest
    pub async fn fetch(&self, client: &reqwest::Client) -> Result<VersionInfo> {
        Ok(client
            .get(&self.url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
}

impl VersionInfo {
    pub async fn fetch_asset_index(&self, client: &reqwest::Client) -> Result<MCAssetIndex> {
        Ok(client
            .get(&self.asset_index.url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
}

/// `PistonRuleMatcher` is an utility for matching argument and library rules
pub struct PistonRuleMatcher {
    pub os: PistonOs,
    pub features: HashMap<String, bool>,
}

impl PistonRuleMatcher {
    /// Creates a piston rule matcher with the provided OS information
    #[must_use]
    pub fn new(os_name: String, os_arch: String, os_version: String) -> Self {
        Self {
            os: PistonOs {
                name: os_name,
                arch: os_arch,
                version: os_version,
            },
            features: HashMap::new(),
        }
    }

    /// Creates a piston rule matcher with empty OS information
    #[must_use]
    pub fn empty() -> Self {
        Self {
            os: PistonOs {
                name: String::new(),
                arch: String::new(),
                version: String::new(),
            },
            features: HashMap::new(),
        }
    }

    /// Creates a rule matcher with OS info from the current system.
    /// Use `PistonRuleMatcher::new()` if OS detection is not desired.
    #[must_use]
    pub fn from_os() -> Self {
        let info = os_info::get();
        let os_name = match info.os_type() {
            os_info::Type::Windows => "windows",
            os_info::Type::Macos => "osx",
            _ => "linux", // Close enough
        }
        .to_owned();

        let os_version = match info.version() {
            os_info::Version::Unknown => String::new(),
            v => v.to_string(),
        };

        Self {
            os: PistonOs {
                name: os_name,
                arch: env::consts::ARCH.to_owned(),
                version: os_version,
            },
            features: HashMap::new(),
        }
    }

    pub fn should_download_library(&self, library: &PistonLibrary) -> Result<bool> {
        self.match_rules(&library.rules)
    }

    /// find classifier from library.
    /// Some(PistonFile) if classifier for matcher exists
    /// None if no classifiers exist/no matches
    #[must_use]
    pub fn get_native_library(&self, library: &PistonLibrary) -> Option<PistonFile> {
        if let Some(native_keys) = &library.natives {
            if let Some(classifier_key) = native_keys.get(&self.os.name) {
                if let Some(map) = &library.downloads.classifiers {
                    return Some(
                        map[&self.process_string(&HashMap::new(), classifier_key)].clone(),
                    );
                }
            }
        }

        None
    }

    pub fn match_rules(&self, rules: &Vec<PistonRule>) -> Result<bool> {
        if rules.is_empty() {
            return Ok(true);
        }

        for rule in rules {
            if !self.match_rule(rule)? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub fn match_rule(&self, rule: &PistonRule) -> Result<bool> {
        Ok(match rule {
            PistonRule::Allow(constraint) => self.match_constraint(constraint)?,
            PistonRule::Disallow(constraint) => {
                // Fuck it
                !self.match_constraint(constraint)?
            }
        })
    }

    pub fn match_constraint(&self, constraint: &PistonRuleConstraints) -> Result<bool> {
        if let Some(os) = &constraint.os {
            if !os.name.is_empty() && os.name != self.os.name {
                return Ok(false);
            }

            if !os.arch.is_empty() && os.arch != self.os.arch {
                return Ok(false);
            }

            if !os.version.is_empty() && !Regex::new(&os.version)?.is_match(&self.os.version) {
                return Ok(false);
            }
        }

        if let Some(feats) = &constraint.features {
            for feat in feats.keys() {
                if !self.features.contains_key(feat) || !self.features[feat] {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }

    pub fn build_args(
        &self,
        args: &[PistonArgument],
        map: &HashMap<String, String>,
    ) -> Result<Vec<String>> {
        let mut list: Vec<String> = vec![];
        for arg in args {
            match arg {
                PistonArgument::Normal(str) => list.push(str.to_owned()),
                PistonArgument::Ruled { rules, value } => {
                    if self.match_rules(rules)? {
                        match value {
                            ArgumentValue::Single(v) => list.push(v.to_owned()),
                            // bad
                            ArgumentValue::Many(li) => {
                                li.iter().for_each(|v| list.push(v.to_owned()));
                            }
                        };
                    }
                }
            }
        }

        Ok(list.iter().map(|s| self.process_string(map, s)).collect())
    }

    #[must_use]
    pub fn process_string(&self, map: &HashMap<String, String>, input: &str) -> String {
        dollar_repl(input, |key| {
            if key == "arch" {
                return Some(self.os.arch.clone());
            }

            map.get(key).cloned()
        })
    }
}

impl PistonLibrary {
    pub async fn download_artifact(&self, client: &reqwest::Client) -> Result<reqwest::Response> {
        self.downloads.download_artifact(client).await
    }

    #[must_use]
    pub async fn download_native(
        &self,
        client: &reqwest::Client,
        native_id: &str,
    ) -> Option<Result<reqwest::Response>> {
        self.downloads.download_native(client, native_id).await
    }

    #[must_use]
    pub fn get_artifact(&self) -> &PistonFile {
        &self.downloads.artifact
    }

    #[must_use]
    pub fn get_native(&self, native_id: &str) -> Option<PistonFile> {
        match &self.downloads.classifiers {
            None => None,
            Some(map) => map.get(native_id).cloned(),
        }
    }

    #[must_use]
    pub fn get_artifact_path(&self) -> Option<String> {
        self.downloads.artifact.path.as_ref().cloned()
    }

    #[must_use]
    pub fn get_native_path(&self, native_id: &str) -> Option<String> {
        self.downloads
            .classifiers
            .as_ref()
            .and_then(|c| c.get(native_id))
            .and_then(|n| n.path.as_ref())
            .cloned()
    }
}

impl PistonLibraryDownload {
    pub async fn download_artifact(&self, client: &reqwest::Client) -> Result<reqwest::Response> {
        self.artifact.download(client).await
    }

    #[must_use]
    pub async fn download_native(
        &self,
        client: &reqwest::Client,
        native_id: &str,
    ) -> Option<Result<reqwest::Response>> {
        let n = self.classifiers.as_ref().and_then(|c| c.get(native_id))?;
        Some(n.download(client).await)
    }
}

impl PistonFile {
    pub async fn download(&self, client: &reqwest::Client) -> Result<reqwest::Response> {
        Ok(client.get(&self.url).send().await?.error_for_status()?)
    }
}
