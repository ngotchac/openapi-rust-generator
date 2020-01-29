use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LibraryPostUpdatedMedia {
    #[serde(rename(deserialize = "Updates"))]
    pub updates: Vec<Box<crate::models::LibraryMediaUpdateInfo>>,
}


