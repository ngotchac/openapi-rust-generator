use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DlnaXmlAttribute {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Value"))]
    pub value: String,
}


