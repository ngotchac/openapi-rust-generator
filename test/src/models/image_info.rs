use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageInfo {
    #[serde(rename(deserialize = "ImageType"))]
    pub image_type: ImageType,
    #[serde(rename(deserialize = "ImageIndex"), skip_serializing_if = "Option::is_none")]
    pub image_index: Option<i32>,
    #[serde(rename(deserialize = "Path"))]
    pub path: String,
    #[serde(rename(deserialize = "Height"), skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(rename(deserialize = "Width"), skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(rename(deserialize = "Size"))]
    pub size: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ImageType {
    #[serde(rename = "Primary")]
    Primary,
    #[serde(rename = "Art")]
    Art,
    #[serde(rename = "Backdrop")]
    Backdrop,
    #[serde(rename = "Banner")]
    Banner,
    #[serde(rename = "Logo")]
    Logo,
    #[serde(rename = "Thumb")]
    Thumb,
    #[serde(rename = "Disc")]
    Disc,
    #[serde(rename = "Box")]
    _Box,
    #[serde(rename = "Screenshot")]
    Screenshot,
    #[serde(rename = "Menu")]
    Menu,
    #[serde(rename = "Chapter")]
    Chapter,
    #[serde(rename = "BoxRear")]
    BoxRear,
    #[serde(rename = "Thumbnail")]
    Thumbnail,
}

