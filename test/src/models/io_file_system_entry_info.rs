use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoFileSystemEntryInfo {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Path"))]
    pub path: String,
    #[serde(rename(deserialize = "Type"))]
    pub _type: Type,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "File")]
    File,
    #[serde(rename = "Directory")]
    Directory,
    #[serde(rename = "NetworkComputer")]
    NetworkComputer,
    #[serde(rename = "NetworkShare")]
    NetworkShare,
}

