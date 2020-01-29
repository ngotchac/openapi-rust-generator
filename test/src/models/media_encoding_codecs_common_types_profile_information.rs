use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaEncodingCodecsCommonTypesProfileInformation {
    #[serde(rename(deserialize = "ShortName"))]
    pub short_name: String,
    #[serde(rename(deserialize = "Description"))]
    pub description: String,
    #[serde(rename(deserialize = "Details"))]
    pub details: String,
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
}


