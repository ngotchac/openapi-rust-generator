use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualFolderInfo {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Locations"))]
    pub locations: Vec<String>,
    #[serde(rename(deserialize = "CollectionType"))]
    pub collection_type: String,
    #[serde(rename(deserialize = "LibraryOptions"))]
    pub library_options: Box<crate::models::ConfigurationLibraryOptions>,
    #[serde(rename(deserialize = "ItemId"))]
    pub item_id: String,
    #[serde(rename(deserialize = "PrimaryImageItemId"))]
    pub primary_image_item_id: String,
    #[serde(rename(deserialize = "RefreshProgress"), skip_serializing_if = "Option::is_none")]
    pub refresh_progress: Option<f64>,
    #[serde(rename(deserialize = "RefreshStatus"))]
    pub refresh_status: String,
}


