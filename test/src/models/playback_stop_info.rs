use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlaybackStopInfo {
    #[serde(rename(deserialize = "NowPlayingQueue"))]
    pub now_playing_queue: Vec<Box<crate::models::QueueItem>>,
    #[serde(rename(deserialize = "PlaylistItemId"))]
    pub playlist_item_id: String,
    #[serde(rename(deserialize = "Item"))]
    pub item: Box<crate::models::BaseItemDto>,
    #[serde(rename(deserialize = "ItemId"))]
    pub item_id: String,
    #[serde(rename(deserialize = "SessionId"))]
    pub session_id: String,
    #[serde(rename(deserialize = "MediaSourceId"))]
    pub media_source_id: String,
    #[serde(rename(deserialize = "PositionTicks"), skip_serializing_if = "Option::is_none")]
    pub position_ticks: Option<i64>,
    #[serde(rename(deserialize = "LiveStreamId"))]
    pub live_stream_id: String,
    #[serde(rename(deserialize = "PlaySessionId"))]
    pub play_session_id: String,
    #[serde(rename(deserialize = "Failed"))]
    pub failed: bool,
    #[serde(rename(deserialize = "NextMediaType"))]
    pub next_media_type: String,
}


