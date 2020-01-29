use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LibraryLibraryOptionsResult {
    #[serde(rename(deserialize = "MetadataSavers"))]
    pub metadata_savers: Vec<Box<crate::models::LibraryLibraryOptionInfo>>,
    #[serde(rename(deserialize = "MetadataReaders"))]
    pub metadata_readers: Vec<Box<crate::models::LibraryLibraryOptionInfo>>,
    #[serde(rename(deserialize = "SubtitleFetchers"))]
    pub subtitle_fetchers: Vec<Box<crate::models::LibraryLibraryOptionInfo>>,
    #[serde(rename(deserialize = "TypeOptions"))]
    pub type_options: Vec<Box<crate::models::LibraryLibraryTypeOptions>>,
}


