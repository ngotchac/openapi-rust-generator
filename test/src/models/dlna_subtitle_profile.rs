use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DlnaSubtitleProfile {
    #[serde(rename(deserialize = "Format"))]
    pub format: String,
    #[serde(rename(deserialize = "Method"))]
    pub method: Method,
    #[serde(rename(deserialize = "DidlMode"))]
    pub didl_mode: String,
    #[serde(rename(deserialize = "Language"))]
    pub language: String,
    #[serde(rename(deserialize = "Container"))]
    pub container: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Method {
    #[serde(rename = "Encode")]
    Encode,
    #[serde(rename = "Embed")]
    Embed,
    #[serde(rename = "External")]
    External,
    #[serde(rename = "Hls")]
    Hls,
}

