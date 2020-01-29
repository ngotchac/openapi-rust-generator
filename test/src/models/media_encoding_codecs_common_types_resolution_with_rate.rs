use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaEncodingCodecsCommonTypesResolutionWithRate {
    #[serde(rename(deserialize = "Width"))]
    pub width: i32,
    #[serde(rename(deserialize = "Height"))]
    pub height: i32,
    #[serde(rename(deserialize = "FrameRate"))]
    pub frame_rate: f64,
    #[serde(rename(deserialize = "Resolution"))]
    pub resolution: Box<crate::models::MediaEncodingCodecsCommonTypesResolution>,
}


