use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncModelSyncProfileOption {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Description"))]
    pub description: String,
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
    #[serde(rename(deserialize = "IsDefault"))]
    pub is_default: bool,
    #[serde(rename(deserialize = "EnableQualityOptions"))]
    pub enable_quality_options: bool,
}


