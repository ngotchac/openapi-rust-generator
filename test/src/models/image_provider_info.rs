use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageProviderInfo {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "SupportedImages"))]
    pub supported_images: SupportedImages,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SupportedImages {
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

