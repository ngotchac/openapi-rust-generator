use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RokuMetadataApiThumbnailSetInfo {
    #[serde(rename(deserialize = "AspectRatio"), skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<f64>,
    #[serde(rename(deserialize = "Thumbnails"))]
    pub thumbnails: Vec<Box<crate::models::RokuMetadataApiThumbnailInfo>>,
}


