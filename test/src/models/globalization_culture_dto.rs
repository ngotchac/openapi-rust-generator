use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GlobalizationCultureDto {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "DisplayName"))]
    pub display_name: String,
    #[serde(rename(deserialize = "TwoLetterISOLanguageName"))]
    pub two_letter_iso_language_name: String,
    #[serde(rename(deserialize = "ThreeLetterISOLanguageName"))]
    pub three_letter_iso_language_name: String,
    #[serde(rename(deserialize = "ThreeLetterISOLanguageNames"))]
    pub three_letter_iso_language_names: Vec<String>,
}


