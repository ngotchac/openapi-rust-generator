use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicSystemInfo {
    #[serde(rename(deserialize = "LocalAddress"))]
    pub local_address: String,
    #[serde(rename(deserialize = "WanAddress"))]
    pub wan_address: String,
    #[serde(rename(deserialize = "ServerName"))]
    pub server_name: String,
    #[serde(rename(deserialize = "Version"))]
    pub version: String,
    #[serde(rename(deserialize = "OperatingSystem"))]
    pub operating_system: String,
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
}


