use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LibraryUpdateLibraryOptions {
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
    #[serde(rename(deserialize = "LibraryOptions"))]
    pub library_options: Box<crate::models::ConfigurationLibraryOptions>,
}


