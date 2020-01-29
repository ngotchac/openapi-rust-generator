use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbyNotificationsApiNotification {
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
    #[serde(rename(deserialize = "UserId"))]
    pub user_id: String,
    #[serde(rename(deserialize = "Date"))]
    pub date: String,
    #[serde(rename(deserialize = "IsRead"))]
    pub is_read: bool,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Description"))]
    pub description: String,
    #[serde(rename(deserialize = "Url"))]
    pub url: String,
    #[serde(rename(deserialize = "Level"))]
    pub level: Level,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Level {
    #[serde(rename = "Normal")]
    Normal,
    #[serde(rename = "Warning")]
    Warning,
    #[serde(rename = "Error")]
    Error,
}

