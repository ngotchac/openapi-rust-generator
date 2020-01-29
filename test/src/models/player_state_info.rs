use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlayerStateInfo {
    #[serde(rename(deserialize = "PositionTicks"), skip_serializing_if = "Option::is_none")]
    pub position_ticks: Option<i64>,
    #[serde(rename(deserialize = "CanSeek"))]
    pub can_seek: bool,
    #[serde(rename(deserialize = "IsPaused"))]
    pub is_paused: bool,
    #[serde(rename(deserialize = "IsMuted"))]
    pub is_muted: bool,
    #[serde(rename(deserialize = "VolumeLevel"), skip_serializing_if = "Option::is_none")]
    pub volume_level: Option<i32>,
    #[serde(rename(deserialize = "AudioStreamIndex"), skip_serializing_if = "Option::is_none")]
    pub audio_stream_index: Option<i32>,
    #[serde(rename(deserialize = "SubtitleStreamIndex"), skip_serializing_if = "Option::is_none")]
    pub subtitle_stream_index: Option<i32>,
    #[serde(rename(deserialize = "MediaSourceId"))]
    pub media_source_id: String,
    #[serde(rename(deserialize = "PlayMethod"))]
    pub play_method: PlayMethod,
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

