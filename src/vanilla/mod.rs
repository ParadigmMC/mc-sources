use std::{env, collections::HashMap};

use os_version::{detect, OsVersion};
use regex::Regex;

use crate::{Result, Error, dollar_repl};

mod manifest;
mod version;
mod assets;

pub use crate::vanilla::{manifest::*, version::*, assets::*};

pub const VERSION_MANIFEST_URL: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

pub async fn fetch_version_manifest(
    client: &reqwest::Client,
) -> Result<VersionManifest> {
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
    pub fn find(&self, id: &str) -> Option<VersionIndex> {
        self.versions
            .iter()
            .find(|v| v.id == id).map(|v| v.clone())
    }

    /// Fetch the latest release's VersionInfo
    pub async fn fetch_latest_release(&self, client: &reqwest::Client) -> Result<VersionInfo> {
        let id = self.latest.release.clone();
        self.fetch(&id, client).await
    }
    
    /// Fetch the latest snapshot's VersionInfo
    pub async fn fetch_latest_snapshot(&self, client: &reqwest::Client) -> Result<VersionInfo> {
        let id = self.latest.snapshot.clone();
        self.fetch(&id, client).await
    }

    /// Fetch the VersionInfo of id
    pub async fn fetch(&self, id: &str, client: &reqwest::Client) -> Result<VersionInfo> {
        self
            .find(id)
            .ok_or(Error::NotFound(id.to_owned()))?.fetch(client).await
    }
}

impl VersionIndex {
    /// Fetch the VersionInfo from the manifest
    pub async fn fetch(&self, client: &reqwest::Client) -> Result<VersionInfo> {
        Ok(client.get(&self.url).send().await?.error_for_status()?.json().await?)
    }
}

impl VersionInfo {
    pub async fn fetch_asset_index(&self, client: &reqwest::Client) -> Result<MCAssetIndex> {
        Ok(client.get(&self.asset_index.url).send().await?.error_for_status()?.json().await?)
    }
}

pub struct PistonRuleMatcher {
    pub os: PistonOs,
    pub features: HashMap<String, bool>,
}

impl PistonRuleMatcher {
    /// Creates an empty rule matcher
    pub fn empty() -> Self {
        Self { os: PistonOs { name: String::new(), arch: String::new(), version: String::new() }, features: HashMap::new() }
    }

    /// Creates a rule matcher with current OS info.
    /// Use PistonRuleMatcher::empty() to get an empty one if this one panics or you dont want OS detection
    pub fn new() -> Result<Self> {
        let os_info = detect().expect("os-version failed");

        let os_name = match os_info {
            OsVersion::Windows(_) => "windows",
            OsVersion::MacOS(_) => "osx",
            _ => "linux",
        }.to_owned();

        let os_version = match os_info {
            OsVersion::Windows(windows) => windows.version,
            OsVersion::MacOS(osx) => osx.version,
            OsVersion::Linux(linux) => linux.version.unwrap_or_default(),
            _ => unimplemented!(),
        };

        Ok(Self {
            os: PistonOs {
                name: os_name,
                arch: env::consts::ARCH.to_owned(),
                version: os_version,
            },
            features: HashMap::new(),
        })
    }

    pub fn should_download_library(&self, library: &PistonLibrary) -> bool {
        self.match_rules(&library.rules)
    }

    /// find classifier from library.
    /// Some(PistonFile) if classifier for matcher exists
    /// None if no classifiers exist/no matches
    pub fn get_native_library(&self, library: &PistonLibrary) -> Option<PistonFile> {
        if let Some(native_keys) = &library.natives {
            if let Some(classifier_key) = native_keys.get(&self.os.name) {
                if let Some(map) = &library.downloads.classifiers {
                    return Some(
                        map[&self.process_string(&HashMap::new(), classifier_key)].clone()
                    )
                }
            }
        }

        None
    }

    pub fn match_rules(&self, rules: &Vec<PistonRule>) -> bool {
        if rules.is_empty() {
            true
        } else {
            for rule in rules.iter() {
                if !self.match_rule(rule) {
                    return false;
                }
            }
            true
        }
    }

    pub fn match_rule(&self, rule: &PistonRule) -> bool {
        match rule {
            PistonRule::Allow(constraint) => {
                if let Some(os) = &constraint.os {
                    if !os.name.is_empty() && os.name != self.os.name {
                        return false;
                    }

                    if !os.arch.is_empty() && os.arch != self.os.arch {
                        return false;
                    }

                    if !os.version.is_empty() && !Regex::new(&os.version).unwrap().is_match(&self.os.version) {
                        return false;
                    }
                }

                if let Some(feats) = &constraint.features {
                    for (feat, _) in feats {
                        if !self.features.contains_key(feat) || !self.features[feat] {
                            return false;
                        }
                    }
                }
            },
            PistonRule::Disallow(_) => {
                // Fuck it
                return !self.match_rule(rule);
            }
        }

        return true;
    }

    pub fn build_args(&self, args: &Vec<PistonArgument>, map: &HashMap<String, String>) -> String {
        let mut list: Vec<String> = vec![];
        for arg in args.iter() {
            match arg {
                PistonArgument::Normal(str) => list.push(str.to_owned()),
                PistonArgument::Ruled { rules, value } => {
                    if self.match_rules(rules) {
                        list.push(match value {
                            ArgumentValue::Single(v) => v.to_owned(),
                            ArgumentValue::Many(li) => li.join(" "),
                        });
                    }
                }
            }
        }

        self.process_string(map, &list.join(" "))
    }

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

    pub async fn download_native(&self, client: &reqwest::Client, native_id: &str) -> Result<reqwest::Response> {
        self.downloads.download_native(client, native_id).await
    }

    pub fn get_artifact(&self) -> &PistonFile {
        &self.downloads.artifact
    }

    pub fn get_native(&self, native_id: &str) -> Option<PistonFile> {
        match &self.downloads.classifiers {
            None => None,
            Some(map) => map.get(native_id).cloned()
        }
    }

    pub fn get_artifact_path(&self) -> String {
        self.downloads.artifact.path.as_ref().unwrap().to_owned()
    }

    pub fn get_native_path(&self, native_id: &str) -> String {
        self.downloads.classifiers.as_ref().unwrap()[native_id].path.as_ref().unwrap().to_owned()
    }
}

impl PistonLibraryDownload {
    pub async fn download_artifact(&self, client: &reqwest::Client) -> Result<reqwest::Response> {
        self.artifact.download(client).await
    }
    
    pub async fn download_native(&self, client: &reqwest::Client, native_id: &str) -> Result<reqwest::Response> {
        self.classifiers.as_ref().unwrap()[native_id].download(client).await
    }
}

impl PistonFile {
    pub async fn download(&self, client: &reqwest::Client) -> Result<reqwest::Response> {
        Ok(client.get(&self.url).send().await?.error_for_status()?)
    }
}
