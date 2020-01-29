use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LibraryAddVirtualFolder {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "CollectionType"))]
    pub collection_type: String,
    #[serde(rename(deserialize = "RefreshLibrary"))]
    pub refresh_library: bool,
    #[serde(rename(deserialize = "Paths"))]
    pub paths: Vec<String>,
    #[serde(rename(deserialize = "LibraryOptions"))]
    pub library_options: Box<crate::models::ConfigurationLibraryOptions>,
}


