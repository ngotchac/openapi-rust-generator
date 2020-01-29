use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DlnaDeviceIdentification {
    #[serde(rename(deserialize = "FriendlyName"))]
    pub friendly_name: String,
    #[serde(rename(deserialize = "ModelNumber"))]
    pub model_number: String,
    #[serde(rename(deserialize = "SerialNumber"))]
    pub serial_number: String,
    #[serde(rename(deserialize = "ModelName"))]
    pub model_name: String,
    #[serde(rename(deserialize = "ModelDescription"))]
    pub model_description: String,
    #[serde(rename(deserialize = "DeviceDescription"))]
    pub device_description: String,
    #[serde(rename(deserialize = "ModelUrl"))]
    pub model_url: String,
    #[serde(rename(deserialize = "Manufacturer"))]
    pub manufacturer: String,
    #[serde(rename(deserialize = "ManufacturerUrl"))]
    pub manufacturer_url: String,
    #[serde(rename(deserialize = "Headers"))]
    pub headers: Vec<Box<crate::models::DlnaHttpHeaderInfo>>,
}


