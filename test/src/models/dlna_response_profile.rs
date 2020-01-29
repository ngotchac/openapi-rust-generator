use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DlnaResponseProfile {
    #[serde(rename(deserialize = "Container"))]
    pub container: String,
    #[serde(rename(deserialize = "AudioCodec"))]
    pub audio_codec: String,
    #[serde(rename(deserialize = "VideoCodec"))]
    pub video_codec: String,
    #[serde(rename(deserialize = "Type"))]
    pub _type: Type,
    #[serde(rename(deserialize = "OrgPn"))]
    pub org_pn: String,
    #[serde(rename(deserialize = "MimeType"))]
    pub mime_type: String,
    #[serde(rename(deserialize = "Conditions"))]
    pub conditions: Vec<Box<crate::models::DlnaProfileCondition>>,
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

