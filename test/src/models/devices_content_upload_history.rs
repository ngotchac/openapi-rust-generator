use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevicesContentUploadHistory {
    #[serde(rename(deserialize = "DeviceId"))]
    pub device_id: String,
    #[serde(rename(deserialize = "FilesUploaded"))]
    pub files_uploaded: Vec<Box<crate::models::DevicesLocalFileInfo>>,
}


