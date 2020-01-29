use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageByNameInfo {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Theme"))]
    pub theme: String,
    #[serde(rename(deserialize = "Context"))]
    pub context: String,
    #[serde(rename(deserialize = "FileLength"))]
    pub file_length: i64,
    #[serde(rename(deserialize = "Format"))]
    pub format: String,
}


