use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetEndPointInfo {
    #[serde(rename(deserialize = "IsLocal"))]
    pub is_local: bool,
    #[serde(rename(deserialize = "IsInNetwork"))]
    pub is_in_network: bool,
}


