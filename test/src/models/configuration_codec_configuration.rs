use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationCodecConfiguration {
    #[serde(rename(deserialize = "IsEnabled"))]
    pub is_enabled: bool,
    #[serde(rename(deserialize = "Priority"))]
    pub priority: i32,
    #[serde(rename(deserialize = "CodecId"))]
    pub codec_id: String,
}


