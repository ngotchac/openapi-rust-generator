use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataEditorInfo {
    #[serde(rename(deserialize = "ParentalRatingOptions"))]
    pub parental_rating_options: Vec<Box<crate::models::ParentalRating>>,
    #[serde(rename(deserialize = "Countries"))]
    pub countries: Vec<Box<crate::models::GlobalizationCountryInfo>>,
    #[serde(rename(deserialize = "Cultures"))]
    pub cultures: Vec<Box<crate::models::GlobalizationCultureDto>>,
    #[serde(rename(deserialize = "ExternalIdInfos"))]
    pub external_id_infos: Vec<Box<crate::models::ExternalIdInfo>>,
}


