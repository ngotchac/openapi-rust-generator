use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PluginsPluginInfo {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Version"))]
    pub version: String,
    #[serde(rename(deserialize = "ConfigurationFileName"))]
    pub configuration_file_name: String,
    #[serde(rename(deserialize = "Description"))]
    pub description: String,
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
    #[serde(rename(deserialize = "ImageTag"))]
    pub image_tag: String,
}


