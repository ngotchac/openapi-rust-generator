use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProvidersMovieInfo {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "MetadataLanguage"))]
    pub metadata_language: String,
    #[serde(rename(deserialize = "MetadataCountryCode"))]
    pub metadata_country_code: String,
    #[serde(rename(deserialize = "ProviderIds"))]
    pub provider_ids: ::std::collections::HashMap<String, String>,
    #[serde(rename(deserialize = "Year"), skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    #[serde(rename(deserialize = "IndexNumber"), skip_serializing_if = "Option::is_none")]
    pub index_number: Option<i32>,
    #[serde(rename(deserialize = "ParentIndexNumber"), skip_serializing_if = "Option::is_none")]
    pub parent_index_number: Option<i32>,
    #[serde(rename(deserialize = "PremiereDate"), skip_serializing_if = "Option::is_none")]
    pub premiere_date: Option<String>,
    #[serde(rename(deserialize = "IsAutomated"))]
    pub is_automated: bool,
}


