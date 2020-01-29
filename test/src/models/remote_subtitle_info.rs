use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoteSubtitleInfo {
    #[serde(rename(deserialize = "ThreeLetterISOLanguageName"))]
    pub three_letter_iso_language_name: String,
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
    #[serde(rename(deserialize = "ProviderName"))]
    pub provider_name: String,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Format"))]
    pub format: String,
    #[serde(rename(deserialize = "Author"))]
    pub author: String,
    #[serde(rename(deserialize = "Comment"))]
    pub comment: String,
    #[serde(rename(deserialize = "DateCreated"), skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(rename(deserialize = "CommunityRating"), skip_serializing_if = "Option::is_none")]
    pub community_rating: Option<f32>,
    #[serde(rename(deserialize = "DownloadCount"), skip_serializing_if = "Option::is_none")]
    pub download_count: Option<i32>,
    #[serde(rename(deserialize = "IsHashMatch"), skip_serializing_if = "Option::is_none")]
    pub is_hash_match: Option<bool>,
    #[serde(rename(deserialize = "IsForced"), skip_serializing_if = "Option::is_none")]
    pub is_forced: Option<bool>,
}


