use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbyReportsApiModelReportResult {
    #[serde(rename(deserialize = "Rows"))]
    pub rows: Vec<Box<crate::models::EmbyReportsApiModelReportRow>>,
    #[serde(rename(deserialize = "Headers"))]
    pub headers: Vec<Box<crate::models::EmbyReportsApiModelReportHeader>>,
    #[serde(rename(deserialize = "Groups"))]
    pub groups: Vec<Box<crate::models::EmbyReportsApiModelReportGroup>>,
    #[serde(rename(deserialize = "TotalRecordCount"))]
    pub total_record_count: i32,
    #[serde(rename(deserialize = "IsGrouped"))]
    pub is_grouped: bool,
}


