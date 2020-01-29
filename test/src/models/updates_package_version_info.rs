use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdatesPackageVersionInfo {
    #[serde(rename(deserialize = "name"))]
    pub name: String,
    #[serde(rename(deserialize = "guid"))]
    pub guid: String,
    #[serde(rename(deserialize = "versionStr"))]
    pub version_str: String,
    #[serde(rename(deserialize = "classification"))]
    pub classification: Classification,
    #[serde(rename(deserialize = "description"))]
    pub description: String,
    #[serde(rename(deserialize = "requiredVersionStr"))]
    pub required_version_str: String,
    #[serde(rename(deserialize = "sourceUrl"))]
    pub source_url: String,
    #[serde(rename(deserialize = "checksum"))]
    pub checksum: String,
    #[serde(rename(deserialize = "targetFilename"))]
    pub target_filename: String,
    #[serde(rename(deserialize = "infoUrl"))]
    pub info_url: String,
    #[serde(rename(deserialize = "runtimes"))]
    pub runtimes: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Classification {
    #[serde(rename = "Release")]
    Release,
    #[serde(rename = "Beta")]
    Beta,
    #[serde(rename = "Dev")]
    Dev,
}

