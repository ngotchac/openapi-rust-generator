use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlaybackReportingApiCustomQuery {
    #[serde(rename(deserialize = "CustomQueryString"))]
    pub custom_query_string: String,
    #[serde(rename(deserialize = "ReplaceUserId"))]
    pub replace_user_id: bool,
}


