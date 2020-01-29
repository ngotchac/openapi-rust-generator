use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncModelItemFileInfo {
    #[serde(rename(deserialize = "Type"))]
    pub _type: Type,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Path"))]
    pub path: String,
    #[serde(rename(deserialize = "ImageType"))]
    pub image_type: ImageType,
    #[serde(rename(deserialize = "Index"))]
    pub index: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Media")]
    Media,
    #[serde(rename = "Image")]
    Image,
    #[serde(rename = "Subtitles")]
    Subtitles,
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

