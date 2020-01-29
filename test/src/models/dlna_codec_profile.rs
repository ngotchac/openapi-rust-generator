use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DlnaCodecProfile {
    #[serde(rename(deserialize = "Type"))]
    pub _type: Type,
    #[serde(rename(deserialize = "Conditions"))]
    pub conditions: Vec<Box<crate::models::DlnaProfileCondition>>,
    #[serde(rename(deserialize = "ApplyConditions"))]
    pub apply_conditions: Vec<Box<crate::models::DlnaProfileCondition>>,
    #[serde(rename(deserialize = "Codec"))]
    pub codec: String,
    #[serde(rename(deserialize = "Container"))]
    pub container: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Video")]
    Video,
    #[serde(rename = "VideoAudio")]
    VideoAudio,
    #[serde(rename = "Audio")]
    Audio,
}

