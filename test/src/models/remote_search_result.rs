use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoteSearchResult {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "ProviderIds"))]
    pub provider_ids: ::std::collections::HashMap<String, String>,
    #[serde(rename(deserialize = "ProductionYear"), skip_serializing_if = "Option::is_none")]
    pub production_year: Option<i32>,
    #[serde(rename(deserialize = "IndexNumber"), skip_serializing_if = "Option::is_none")]
    pub index_number: Option<i32>,
    #[serde(rename(deserialize = "IndexNumberEnd"), skip_serializing_if = "Option::is_none")]
    pub index_number_end: Option<i32>,
    #[serde(rename(deserialize = "ParentIndexNumber"), skip_serializing_if = "Option::is_none")]
    pub parent_index_number: Option<i32>,
    #[serde(rename(deserialize = "PremiereDate"), skip_serializing_if = "Option::is_none")]
    pub premiere_date: Option<String>,
    #[serde(rename(deserialize = "ImageUrl"))]
    pub image_url: String,
    #[serde(rename(deserialize = "SearchProviderName"))]
    pub search_provider_name: String,
    #[serde(rename(deserialize = "GameSystem"))]
    pub game_system: String,
    #[serde(rename(deserialize = "Overview"))]
    pub overview: String,
    #[serde(rename(deserialize = "AlbumArtist"))]
    pub album_artist: Box<crate::models::RemoteSearchResult>,
    #[serde(rename(deserialize = "Artists"))]
    pub artists: Vec<Box<crate::models::RemoteSearchResult>>,
}


