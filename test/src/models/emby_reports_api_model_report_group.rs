use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbyReportsApiModelReportGroup {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Rows"))]
    pub rows: Vec<Box<crate::models::EmbyReportsApiModelReportRow>>,
}


