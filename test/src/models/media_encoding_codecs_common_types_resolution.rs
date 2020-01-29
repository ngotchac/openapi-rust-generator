use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaEncodingCodecsCommonTypesResolution {
    #[serde(rename(deserialize = "Width"))]
    pub width: i32,
    #[serde(rename(deserialize = "Height"))]
    pub height: i32,
}


