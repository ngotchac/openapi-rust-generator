use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LiveTvListingsProviderInfo {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "SetupUrl"))]
    pub setup_url: String,
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
    #[serde(rename(deserialize = "Type"))]
    pub _type: String,
    #[serde(rename(deserialize = "Username"))]
    pub username: String,
    #[serde(rename(deserialize = "Password"))]
    pub password: String,
    #[serde(rename(deserialize = "ListingsId"))]
    pub listings_id: String,
    #[serde(rename(deserialize = "ZipCode"))]
    pub zip_code: String,
    #[serde(rename(deserialize = "Country"))]
    pub country: String,
    #[serde(rename(deserialize = "Path"))]
    pub path: String,
    #[serde(rename(deserialize = "EnabledTuners"))]
    pub enabled_tuners: Vec<String>,
    #[serde(rename(deserialize = "EnableAllTuners"))]
    pub enable_all_tuners: bool,
    #[serde(rename(deserialize = "NewsCategories"))]
    pub news_categories: Vec<String>,
    #[serde(rename(deserialize = "SportsCategories"))]
    pub sports_categories: Vec<String>,
    #[serde(rename(deserialize = "KidsCategories"))]
    pub kids_categories: Vec<String>,
    #[serde(rename(deserialize = "MovieCategories"))]
    pub movie_categories: Vec<String>,
    #[serde(rename(deserialize = "ChannelMappings"))]
    pub channel_mappings: Vec<Box<crate::models::NameValuePair>>,
    #[serde(rename(deserialize = "MoviePrefix"))]
    pub movie_prefix: String,
    #[serde(rename(deserialize = "PreferredLanguage"))]
    pub preferred_language: String,
    #[serde(rename(deserialize = "UserAgent"))]
    pub user_agent: String,
}


