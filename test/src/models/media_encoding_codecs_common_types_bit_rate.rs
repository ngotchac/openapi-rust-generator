use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaEncodingCodecsCommonTypesBitRate {
    #[serde(rename(deserialize = "bps"))]
    pub bps: i64,
    #[serde(rename(deserialize = "kbps"))]
    pub kbps: f64,
    #[serde(rename(deserialize = "Mbps"))]
    pub mbps: f64,
}


