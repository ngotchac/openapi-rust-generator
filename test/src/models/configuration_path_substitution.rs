use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationPathSubstitution {
    #[serde(rename(deserialize = "From"))]
    pub from: String,
    #[serde(rename(deserialize = "To"))]
    pub to: String,
}


