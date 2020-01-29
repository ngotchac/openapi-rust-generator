use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DlnaDirectPlayProfile {
    #[serde(rename(deserialize = "Container"))]
    pub container: String,
    #[serde(rename(deserialize = "AudioCodec"))]
    pub audio_codec: String,
    #[serde(rename(deserialize = "VideoCodec"))]
    pub video_codec: String,
    #[serde(rename(deserialize = "Type"))]
    pub _type: Type,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Audio")]
    Audio,
    #[serde(rename = "Video")]
    Video,
    #[serde(rename = "Photo")]
    Photo,
}

