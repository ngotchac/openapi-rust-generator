use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdatesInstallationInfo {
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "AssemblyGuid"))]
    pub assembly_guid: String,
    #[serde(rename(deserialize = "Version"))]
    pub version: String,
    #[serde(rename(deserialize = "UpdateClass"))]
    pub update_class: UpdateClass,
    #[serde(rename(deserialize = "PercentComplete"), skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<f64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UpdateClass {
    #[serde(rename = "Release")]
    Release,
    #[serde(rename = "Beta")]
    Beta,
    #[serde(rename = "Dev")]
    Dev,
}

