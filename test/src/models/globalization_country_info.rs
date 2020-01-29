use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GlobalizationCountryInfo {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "DisplayName"))]
    pub display_name: String,
    #[serde(rename(deserialize = "TwoLetterISORegionName"))]
    pub two_letter_iso_region_name: String,
    #[serde(rename(deserialize = "ThreeLetterISORegionName"))]
    pub three_letter_iso_region_name: String,
}


