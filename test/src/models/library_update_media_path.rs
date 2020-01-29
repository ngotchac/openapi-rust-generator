use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LibraryUpdateMediaPath {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "PathInfo"))]
    pub path_info: Box<crate::models::ConfigurationMediaPathInfo>,
}


