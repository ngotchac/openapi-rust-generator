use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationTypeOptions {
    #[serde(rename(deserialize = "Type"))]
    pub _type: String,
    #[serde(rename(deserialize = "MetadataFetchers"))]
    pub metadata_fetchers: Vec<String>,
    #[serde(rename(deserialize = "MetadataFetcherOrder"))]
    pub metadata_fetcher_order: Vec<String>,
    #[serde(rename(deserialize = "ImageFetchers"))]
    pub image_fetchers: Vec<String>,
    #[serde(rename(deserialize = "ImageFetcherOrder"))]
    pub image_fetcher_order: Vec<String>,
    #[serde(rename(deserialize = "ImageOptions"))]
    pub image_options: Vec<Box<crate::models::ConfigurationImageOption>>,
}


