use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaEncodingCodecsCommonTypesProfileLevelInformation {
    #[serde(rename(deserialize = "Profile"))]
    pub profile: Box<crate::models::MediaEncodingCodecsCommonTypesProfileInformation>,
    #[serde(rename(deserialize = "Level"))]
    pub level: Box<crate::models::MediaEncodingCodecsCommonTypesLevelInformation>,
}


