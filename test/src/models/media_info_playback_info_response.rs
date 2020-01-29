use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaInfoPlaybackInfoResponse {
    #[serde(rename(deserialize = "MediaSources"))]
    pub media_sources: Vec<Box<crate::models::MediaSourceInfo>>,
    #[serde(rename(deserialize = "PlaySessionId"))]
    pub play_session_id: String,
    #[serde(rename(deserialize = "ErrorCode"))]
    pub error_code: ErrorCode,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ErrorCode {
    #[serde(rename = "NotAllowed")]
    NotAllowed,
    #[serde(rename = "NoCompatibleStream")]
    NoCompatibleStream,
    #[serde(rename = "RateLimitExceeded")]
    RateLimitExceeded,
}

