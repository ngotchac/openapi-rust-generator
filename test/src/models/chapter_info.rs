use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChapterInfo {
    #[serde(rename(deserialize = "StartPositionTicks"))]
    pub start_position_ticks: i64,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "ImageTag"))]
    pub image_tag: String,
}


