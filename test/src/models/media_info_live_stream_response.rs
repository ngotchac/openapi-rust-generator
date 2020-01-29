use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaInfoLiveStreamResponse {
    #[serde(rename(deserialize = "MediaSource"))]
    pub media_source: Box<crate::models::MediaSourceInfo>,
}


