use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LiveTvGuideInfo {
    #[serde(rename(deserialize = "StartDate"))]
    pub start_date: String,
    #[serde(rename(deserialize = "EndDate"))]
    pub end_date: String,
}


