use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityLogEntry {
    #[serde(rename(deserialize = "Id"))]
    pub id: i64,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Overview"))]
    pub overview: String,
    #[serde(rename(deserialize = "ShortOverview"))]
    pub short_overview: String,
    #[serde(rename(deserialize = "Type"))]
    pub _type: String,
    #[serde(rename(deserialize = "ItemId"))]
    pub item_id: String,
    #[serde(rename(deserialize = "Date"))]
    pub date: String,
    #[serde(rename(deserialize = "UserId"))]
    pub user_id: String,
    #[serde(rename(deserialize = "UserPrimaryImageTag"))]
    pub user_primary_image_tag: String,
    #[serde(rename(deserialize = "Severity"))]
    pub severity: Severity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Severity {
    #[serde(rename = "Info")]
    Info,
    #[serde(rename = "Debug")]
    Debug,
    #[serde(rename = "Warn")]
    Warn,
    #[serde(rename = "Error")]
    Error,
    #[serde(rename = "Fatal")]
    Fatal,
}

