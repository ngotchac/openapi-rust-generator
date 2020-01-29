use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LibraryAddMediaPath {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Path"))]
    pub path: String,
    #[serde(rename(deserialize = "PathInfo"))]
    pub path_info: Box<crate::models::ConfigurationMediaPathInfo>,
    #[serde(rename(deserialize = "RefreshLibrary"))]
    pub refresh_library: bool,
}


