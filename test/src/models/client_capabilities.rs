use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientCapabilities {
    #[serde(rename(deserialize = "PlayableMediaTypes"))]
    pub playable_media_types: Vec<String>,
    #[serde(rename(deserialize = "SupportedCommands"))]
    pub supported_commands: Vec<String>,
    #[serde(rename(deserialize = "SupportsMediaControl"))]
    pub supports_media_control: bool,
    #[serde(rename(deserialize = "PushToken"))]
    pub push_token: String,
    #[serde(rename(deserialize = "PushTokenType"))]
    pub push_token_type: String,
    #[serde(rename(deserialize = "SupportsPersistentIdentifier"))]
    pub supports_persistent_identifier: bool,
    #[serde(rename(deserialize = "SupportsSync"))]
    pub supports_sync: bool,
    #[serde(rename(deserialize = "DeviceProfile"))]
    pub device_profile: Box<crate::models::DlnaDeviceProfile>,
    #[serde(rename(deserialize = "IconUrl"))]
    pub icon_url: String,
    #[serde(rename(deserialize = "AppId"))]
    pub app_id: String,
}


