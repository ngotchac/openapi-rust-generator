use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefaultDirectoryBrowserInfo {
    #[serde(rename(deserialize = "Path"))]
    pub path: String,
}


