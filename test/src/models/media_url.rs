use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaUrl {
    #[serde(rename(deserialize = "Url"))]
    pub url: String,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
}


