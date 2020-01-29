use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbyNotificationsApiNotificationsSummary {
    #[serde(rename(deserialize = "UnreadCount"))]
    pub unread_count: i32,
    #[serde(rename(deserialize = "MaxUnreadNotificationLevel"))]
    pub max_unread_notification_level: MaxUnreadNotificationLevel,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MaxUnreadNotificationLevel {
    #[serde(rename = "Normal")]
    Normal,
    #[serde(rename = "Warning")]
    Warning,
    #[serde(rename = "Error")]
    Error,
}

