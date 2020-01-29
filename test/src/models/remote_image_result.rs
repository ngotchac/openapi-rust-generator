use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoteImageResult {
    #[serde(rename(deserialize = "Images"))]
    pub images: Vec<Box<crate::models::RemoteImageInfo>>,
    #[serde(rename(deserialize = "TotalRecordCount"))]
    pub total_record_count: i32,
    #[serde(rename(deserialize = "Providers"))]
    pub providers: Vec<String>,
}


