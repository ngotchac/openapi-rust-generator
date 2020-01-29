use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LibraryLibraryTypeOptions {
    #[serde(rename(deserialize = "Type"))]
    pub _type: String,
    #[serde(rename(deserialize = "MetadataFetchers"))]
    pub metadata_fetchers: Vec<Box<crate::models::LibraryLibraryOptionInfo>>,
    #[serde(rename(deserialize = "ImageFetchers"))]
    pub image_fetchers: Vec<Box<crate::models::LibraryLibraryOptionInfo>>,
    #[serde(rename(deserialize = "SupportedImageTypes"))]
    pub supported_image_types: SupportedImageTypes,
    #[serde(rename(deserialize = "DefaultImageOptions"))]
    pub default_image_options: Vec<Box<crate::models::ConfigurationImageOption>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SupportedImageTypes {
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

