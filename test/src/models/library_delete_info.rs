use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LibraryDeleteInfo {
    #[serde(rename(deserialize = "Paths"))]
    pub paths: Vec<String>,
}


