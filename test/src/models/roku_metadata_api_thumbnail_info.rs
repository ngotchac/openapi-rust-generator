use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RokuMetadataApiThumbnailInfo {
    #[serde(rename(deserialize = "PositionTicks"))]
    pub position_ticks: i64,
    #[serde(rename(deserialize = "ImageTag"))]
    pub image_tag: String,
}


