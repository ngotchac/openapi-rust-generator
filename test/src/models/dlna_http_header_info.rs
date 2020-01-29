use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DlnaHttpHeaderInfo {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Value"))]
    pub value: String,
    #[serde(rename(deserialize = "Match"))]
    pub _match: Match,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Match {
    #[serde(rename = "Equals")]
    Equals,
    #[serde(rename = "Regex")]
    Regex,
    #[serde(rename = "Substring")]
    Substring,
}

