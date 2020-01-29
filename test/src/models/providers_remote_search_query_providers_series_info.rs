use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProvidersRemoteSearchQueryProvidersSeriesInfo {
    #[serde(rename(deserialize = "SearchInfo"))]
    pub search_info: Box<crate::models::ProvidersSeriesInfo>,
    #[serde(rename(deserialize = "ItemId"))]
    pub item_id: i64,
    #[serde(rename(deserialize = "SearchProviderName"))]
    pub search_provider_name: String,
    #[serde(rename(deserialize = "IncludeDisabledProviders"))]
    pub include_disabled_providers: bool,
}


