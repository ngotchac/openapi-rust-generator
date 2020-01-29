use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LiveTvSetChannelMapping {
    #[serde(rename(deserialize = "TunerChannelId"))]
    pub tuner_channel_id: String,
    #[serde(rename(deserialize = "ProviderChannelId"))]
    pub provider_channel_id: String,
}


