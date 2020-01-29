use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserItemDataDto {
    #[serde(rename(deserialize = "Rating"), skip_serializing_if = "Option::is_none")]
    pub rating: Option<f64>,
    #[serde(rename(deserialize = "PlayedPercentage"), skip_serializing_if = "Option::is_none")]
    pub played_percentage: Option<f64>,
    #[serde(rename(deserialize = "UnplayedItemCount"), skip_serializing_if = "Option::is_none")]
    pub unplayed_item_count: Option<i32>,
    #[serde(rename(deserialize = "PlaybackPositionTicks"))]
    pub playback_position_ticks: i64,
    #[serde(rename(deserialize = "PlayCount"))]
    pub play_count: i32,
    #[serde(rename(deserialize = "IsFavorite"))]
    pub is_favorite: bool,
    #[serde(rename(deserialize = "Likes"), skip_serializing_if = "Option::is_none")]
    pub likes: Option<bool>,
    #[serde(rename(deserialize = "LastPlayedDate"), skip_serializing_if = "Option::is_none")]
    pub last_played_date: Option<String>,
    #[serde(rename(deserialize = "Played"))]
    pub played: bool,
    #[serde(rename(deserialize = "Key"))]
    pub key: String,
    #[serde(rename(deserialize = "ItemId"))]
    pub item_id: String,
}


