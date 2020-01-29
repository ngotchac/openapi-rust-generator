use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoteImageInfo {
    #[serde(rename(deserialize = "ProviderName"))]
    pub provider_name: String,
    #[serde(rename(deserialize = "Url"))]
    pub url: String,
    #[serde(rename(deserialize = "ThumbnailUrl"))]
    pub thumbnail_url: String,
    #[serde(rename(deserialize = "Height"), skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(rename(deserialize = "Width"), skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(rename(deserialize = "CommunityRating"), skip_serializing_if = "Option::is_none")]
    pub community_rating: Option<f64>,
    #[serde(rename(deserialize = "VoteCount"), skip_serializing_if = "Option::is_none")]
    pub vote_count: Option<i32>,
    #[serde(rename(deserialize = "Language"))]
    pub language: String,
    #[serde(rename(deserialize = "Type"))]
    pub _type: Type,
    #[serde(rename(deserialize = "RatingType"))]
    pub rating_type: RatingType,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Primary")]
    Primary,
    #[serde(rename = "Art")]
    Art,
    #[serde(rename = "Backdrop")]
    Backdrop,
    #[serde(rename = "Banner")]
    Banner,
    #[serde(rename = "Logo")]
    Logo,
    #[serde(rename = "Thumb")]
    Thumb,
    #[serde(rename = "Disc")]
    Disc,
    #[serde(rename = "Box")]
    _Box,
    #[serde(rename = "Screenshot")]
    Screenshot,
    #[serde(rename = "Menu")]
    Menu,
    #[serde(rename = "Chapter")]
    Chapter,
    #[serde(rename = "BoxRear")]
    BoxRear,
    #[serde(rename = "Thumbnail")]
    Thumbnail,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RatingType {
    #[serde(rename = "Score")]
    Score,
    #[serde(rename = "Likes")]
    Likes,
}

