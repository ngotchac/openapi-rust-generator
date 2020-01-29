use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlayRequest {
    #[serde(rename(deserialize = "ControllingUserId"))]
    pub controlling_user_id: String,
    #[serde(rename(deserialize = "SubtitleStreamIndex"), skip_serializing_if = "Option::is_none")]
    pub subtitle_stream_index: Option<i32>,
    #[serde(rename(deserialize = "AudioStreamIndex"), skip_serializing_if = "Option::is_none")]
    pub audio_stream_index: Option<i32>,
    #[serde(rename(deserialize = "MediaSourceId"))]
    pub media_source_id: String,
    #[serde(rename(deserialize = "StartIndex"), skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}


