use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryResultNewsNewsItem {
    #[serde(rename(deserialize = "Items"))]
    pub items: Vec<Box<crate::models::NewsNewsItem>>,
    #[serde(rename(deserialize = "TotalRecordCount"))]
    pub total_record_count: i32,
}


