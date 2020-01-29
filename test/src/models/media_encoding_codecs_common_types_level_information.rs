use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaEncodingCodecsCommonTypesLevelInformation {
    #[serde(rename(deserialize = "ShortName"))]
    pub short_name: String,
    #[serde(rename(deserialize = "Description"))]
    pub description: String,
    #[serde(rename(deserialize = "Ordinal"), skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<i32>,
    #[serde(rename(deserialize = "MaxBitRate"))]
    pub max_bit_rate: Box<crate::models::MediaEncodingCodecsCommonTypesBitRate>,
    #[serde(rename(deserialize = "MaxBitRateDisplay"))]
    pub max_bit_rate_display: String,
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
    #[serde(rename(deserialize = "ResolutionRates"))]
    pub resolution_rates: Vec<Box<crate::models::MediaEncodingCodecsCommonTypesResolutionWithRate>>,
    #[serde(rename(deserialize = "ResolutionRateStrings"))]
    pub resolution_rate_strings: Vec<String>,
    #[serde(rename(deserialize = "ResolutionRatesDisplay"))]
    pub resolution_rates_display: String,
}


