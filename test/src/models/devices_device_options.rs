use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevicesDeviceOptions {
    #[serde(rename(deserialize = "CustomName"))]
    pub custom_name: String,
}


