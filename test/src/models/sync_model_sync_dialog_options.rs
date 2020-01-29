use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncModelSyncDialogOptions {
    #[serde(rename(deserialize = "Targets"))]
    pub targets: Vec<Box<crate::models::SyncSyncTarget>>,
    #[serde(rename(deserialize = "Options"))]
    pub options: Options,
    #[serde(rename(deserialize = "QualityOptions"))]
    pub quality_options: Vec<Box<crate::models::SyncModelSyncQualityOption>>,
    #[serde(rename(deserialize = "ProfileOptions"))]
    pub profile_options: Vec<Box<crate::models::SyncModelSyncProfileOption>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Options {
    #[serde(rename = "Name")]
    Name,
    #[serde(rename = "Quality")]
    Quality,
    #[serde(rename = "UnwatchedOnly")]
    UnwatchedOnly,
    #[serde(rename = "SyncNewContent")]
    SyncNewContent,
    #[serde(rename = "ItemLimit")]
    ItemLimit,
    #[serde(rename = "Profile")]
    Profile,
}

