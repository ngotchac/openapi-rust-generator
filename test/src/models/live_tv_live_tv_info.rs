use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LiveTvLiveTvInfo {
    #[serde(rename(deserialize = "Services"))]
    pub services: Vec<Box<crate::models::LiveTvLiveTvServiceInfo>>,
    #[serde(rename(deserialize = "IsEnabled"))]
    pub is_enabled: bool,
    #[serde(rename(deserialize = "EnabledUsers"))]
    pub enabled_users: Vec<String>,
}


