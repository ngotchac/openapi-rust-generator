use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbyNotificationsApiNotificationResult {
    #[serde(rename(deserialize = "Notifications"))]
    pub notifications: Vec<Box<crate::models::EmbyNotificationsApiNotification>>,
    #[serde(rename(deserialize = "TotalRecordCount"))]
    pub total_record_count: i32,
}


