use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryResultUserLibraryOfficialRatingItem {
    #[serde(rename(deserialize = "Items"))]
    pub items: Vec<Box<crate::models::UserLibraryOfficialRatingItem>>,
    #[serde(rename(deserialize = "TotalRecordCount"))]
    pub total_record_count: i32,
}


