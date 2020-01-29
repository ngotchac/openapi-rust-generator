use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LiveTvLiveTvServiceInfo {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "HomePageUrl"))]
    pub home_page_url: String,
    #[serde(rename(deserialize = "Status"))]
    pub status: Status,
    #[serde(rename(deserialize = "StatusMessage"))]
    pub status_message: String,
    #[serde(rename(deserialize = "Version"))]
    pub version: String,
    #[serde(rename(deserialize = "HasUpdateAvailable"))]
    pub has_update_available: bool,
    #[serde(rename(deserialize = "IsVisible"))]
    pub is_visible: bool,
    #[serde(rename(deserialize = "Tuners"))]
    pub tuners: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Ok")]
    Ok,
    #[serde(rename = "Unavailable")]
    Unavailable,
}

