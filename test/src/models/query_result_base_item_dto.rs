use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryResultBaseItemDto {
    #[serde(rename(deserialize = "Items"))]
    pub items: Vec<Box<crate::models::BaseItemDto>>,
    #[serde(rename(deserialize = "TotalRecordCount"))]
    pub total_record_count: i32,
}


