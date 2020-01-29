use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchSearchHintResult {
    #[serde(rename(deserialize = "SearchHints"))]
    pub search_hints: Vec<Box<crate::models::SearchSearchHint>>,
    #[serde(rename(deserialize = "TotalRecordCount"))]
    pub total_record_count: i32,
}


