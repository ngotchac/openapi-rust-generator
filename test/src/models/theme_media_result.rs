use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThemeMediaResult {
    #[serde(rename(deserialize = "OwnerId"))]
    pub owner_id: i64,
    #[serde(rename(deserialize = "Items"))]
    pub items: Vec<Box<crate::models::BaseItemDto>>,
    #[serde(rename(deserialize = "TotalRecordCount"))]
    pub total_record_count: i32,
}


