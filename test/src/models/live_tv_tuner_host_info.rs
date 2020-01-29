use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LiveTvTunerHostInfo {
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
    #[serde(rename(deserialize = "Url"))]
    pub url: String,
    #[serde(rename(deserialize = "Type"))]
    pub _type: String,
    #[serde(rename(deserialize = "DeviceId"))]
    pub device_id: String,
    #[serde(rename(deserialize = "FriendlyName"))]
    pub friendly_name: String,
    #[serde(rename(deserialize = "ImportFavoritesOnly"))]
    pub import_favorites_only: bool,
    #[serde(rename(deserialize = "AllowHWTranscoding"))]
    pub allow_hw_transcoding: bool,
    #[serde(rename(deserialize = "Source"))]
    pub source: String,
    #[serde(rename(deserialize = "TunerCount"))]
    pub tuner_count: i32,
    #[serde(rename(deserialize = "UserAgent"))]
    pub user_agent: String,
}


