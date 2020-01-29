use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserDto {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "ServerId"))]
    pub server_id: String,
    #[serde(rename(deserialize = "ServerName"))]
    pub server_name: String,
    #[serde(rename(deserialize = "ConnectUserName"))]
    pub connect_user_name: String,
    #[serde(rename(deserialize = "ConnectLinkType"))]
    pub connect_link_type: ConnectLinkType,
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
    #[serde(rename(deserialize = "PrimaryImageTag"))]
    pub primary_image_tag: String,
    #[serde(rename(deserialize = "HasPassword"))]
    pub has_password: bool,
    #[serde(rename(deserialize = "HasConfiguredPassword"))]
    pub has_configured_password: bool,
    #[serde(rename(deserialize = "HasConfiguredEasyPassword"))]
    pub has_configured_easy_password: bool,
    #[serde(rename(deserialize = "EnableAutoLogin"), skip_serializing_if = "Option::is_none")]
    pub enable_auto_login: Option<bool>,
    #[serde(rename(deserialize = "LastLoginDate"), skip_serializing_if = "Option::is_none")]
    pub last_login_date: Option<String>,
    #[serde(rename(deserialize = "LastActivityDate"), skip_serializing_if = "Option::is_none")]
    pub last_activity_date: Option<String>,
    #[serde(rename(deserialize = "Configuration"))]
    pub configuration: Box<crate::models::ConfigurationUserConfiguration>,
    #[serde(rename(deserialize = "Policy"))]
    pub policy: Box<crate::models::UsersUserPolicy>,
    #[serde(rename(deserialize = "PrimaryImageAspectRatio"), skip_serializing_if = "Option::is_none")]
    pub primary_image_aspect_ratio: Option<f64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConnectLinkType {
    #[serde(rename = "LinkedUser")]
    LinkedUser,
    #[serde(rename = "Guest")]
    Guest,
}

