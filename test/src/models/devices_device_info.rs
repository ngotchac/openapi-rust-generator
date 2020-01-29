use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevicesDeviceInfo {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
    #[serde(rename(deserialize = "LastUserName"))]
    pub last_user_name: String,
    #[serde(rename(deserialize = "AppName"))]
    pub app_name: String,
    #[serde(rename(deserialize = "AppVersion"))]
    pub app_version: String,
    #[serde(rename(deserialize = "LastUserId"))]
    pub last_user_id: String,
    #[serde(rename(deserialize = "DateLastActivity"))]
    pub date_last_activity: String,
    #[serde(rename(deserialize = "IconUrl"))]
    pub icon_url: String,
}


