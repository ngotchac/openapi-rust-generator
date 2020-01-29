use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlaybackStartInfo {
    #[serde(rename(deserialize = "CanSeek"))]
    pub can_seek: bool,
    #[serde(rename(deserialize = "Item"))]
    pub item: Box<crate::models::BaseItemDto>,
    #[serde(rename(deserialize = "NowPlayingQueue"))]
    pub now_playing_queue: Vec<Box<crate::models::QueueItem>>,
    #[serde(rename(deserialize = "PlaylistItemId"))]
    pub playlist_item_id: String,
    #[serde(rename(deserialize = "ItemId"))]
    pub item_id: String,
    #[serde(rename(deserialize = "SessionId"))]
    pub session_id: String,
    #[serde(rename(deserialize = "MediaSourceId"))]
    pub media_source_id: String,
    #[serde(rename(deserialize = "AudioStreamIndex"), skip_serializing_if = "Option::is_none")]
    pub audio_stream_index: Option<i32>,
    #[serde(rename(deserialize = "SubtitleStreamIndex"), skip_serializing_if = "Option::is_none")]
    pub subtitle_stream_index: Option<i32>,
    #[serde(rename(deserialize = "IsPaused"))]
    pub is_paused: bool,
    #[serde(rename(deserialize = "IsMuted"))]
    pub is_muted: bool,
    #[serde(rename(deserialize = "PositionTicks"), skip_serializing_if = "Option::is_none")]
    pub position_ticks: Option<i64>,
    #[serde(rename(deserialize = "RunTimeTicks"), skip_serializing_if = "Option::is_none")]
    pub run_time_ticks: Option<i64>,
    #[serde(rename(deserialize = "PlaybackStartTimeTicks"), skip_serializing_if = "Option::is_none")]
    pub playback_start_time_ticks: Option<i64>,
    #[serde(rename(deserialize = "VolumeLevel"), skip_serializing_if = "Option::is_none")]
    pub volume_level: Option<i32>,
    #[serde(rename(deserialize = "Brightness"), skip_serializing_if = "Option::is_none")]
    pub brightness: Option<i32>,
    #[serde(rename(deserialize = "AspectRatio"))]
    pub aspect_ratio: String,
    #[serde(rename(deserialize = "PlayMethod"))]
    pub play_method: PlayMethod,
    #[serde(rename(deserialize = "LiveStreamId"))]
    pub live_stream_id: String,
    #[serde(rename(deserialize = "PlaySessionId"))]
    pub play_session_id: String,
    #[serde(rename(deserialize = "RepeatMode"))]
    pub repeat_mode: RepeatMode,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlayMethod {
    #[serde(rename = "Transcode")]
    Transcode,
    #[serde(rename = "DirectStream")]
    DirectStream,
    #[serde(rename = "DirectPlay")]
    DirectPlay,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RepeatMode {
    #[serde(rename = "RepeatNone")]
    RepeatNone,
    #[serde(rename = "RepeatAll")]
    RepeatAll,
    #[serde(rename = "RepeatOne")]
    RepeatOne,
}

