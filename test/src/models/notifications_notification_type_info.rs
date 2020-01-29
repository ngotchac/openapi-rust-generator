use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationsNotificationTypeInfo {
    #[serde(rename(deserialize = "Type"))]
    pub _type: String,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Enabled"))]
    pub enabled: bool,
    #[serde(rename(deserialize = "Category"))]
    pub category: String,
    #[serde(rename(deserialize = "IsBasedOnUserEvent"))]
    pub is_based_on_user_event: bool,
}


