use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WakeOnLanInfo {
    #[serde(rename(deserialize = "MacAddress"))]
    pub mac_address: String,
    #[serde(rename(deserialize = "BroadcastAddress"))]
    pub broadcast_address: String,
    #[serde(rename(deserialize = "Port"))]
    pub port: i32,
}


