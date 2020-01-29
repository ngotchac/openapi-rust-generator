use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TasksTaskResult {
    #[serde(rename(deserialize = "StartTimeUtc"))]
    pub start_time_utc: String,
    #[serde(rename(deserialize = "EndTimeUtc"))]
    pub end_time_utc: String,
    #[serde(rename(deserialize = "Status"))]
    pub status: Status,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Key"))]
    pub key: String,
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
    #[serde(rename(deserialize = "ErrorMessage"))]
    pub error_message: String,
    #[serde(rename(deserialize = "LongErrorMessage"))]
    pub long_error_message: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Completed")]
    Completed,
    #[serde(rename = "Failed")]
    Failed,
    #[serde(rename = "Cancelled")]
    Cancelled,
    #[serde(rename = "Aborted")]
    Aborted,
}

