use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionSessionInfo {
    #[serde(rename(deserialize = "PlayState"))]
    pub play_state: Box<crate::models::PlayerStateInfo>,
    #[serde(rename(deserialize = "AdditionalUsers"))]
    pub additional_users: Vec<Box<crate::models::SessionUserInfo>>,
    #[serde(rename(deserialize = "Capabilities"))]
    pub capabilities: Box<crate::models::ClientCapabilities>,
    #[serde(rename(deserialize = "RemoteEndPoint"))]
    pub remote_end_point: String,
    #[serde(rename(deserialize = "PlayableMediaTypes"))]
    pub playable_media_types: Vec<String>,
    #[serde(rename(deserialize = "PlaylistItemId"))]
    pub playlist_item_id: String,
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
    #[serde(rename(deserialize = "ServerId"))]
    pub server_id: String,
    #[serde(rename(deserialize = "UserId"))]
    pub user_id: String,
    #[serde(rename(deserialize = "UserName"))]
    pub user_name: String,
    #[serde(rename(deserialize = "UserPrimaryImageTag"))]
    pub user_primary_image_tag: String,
    #[serde(rename(deserialize = "Client"))]
    pub client: String,
    #[serde(rename(deserialize = "LastActivityDate"))]
    pub last_activity_date: String,
    #[serde(rename(deserialize = "DeviceName"))]
    pub device_name: String,
    #[serde(rename(deserialize = "DeviceType"))]
    pub device_type: String,
    #[serde(rename(deserialize = "NowPlayingItem"))]
    pub now_playing_item: Box<crate::models::BaseItemDto>,
    #[serde(rename(deserialize = "DeviceId"))]
    pub device_id: String,
    #[serde(rename(deserialize = "ApplicationVersion"))]
    pub application_version: String,
    #[serde(rename(deserialize = "AppIconUrl"))]
    pub app_icon_url: String,
    #[serde(rename(deserialize = "SupportedCommands"))]
    pub supported_commands: Vec<String>,
    #[serde(rename(deserialize = "TranscodingInfo"))]
    pub transcoding_info: Box<crate::models::TranscodingInfo>,
    #[serde(rename(deserialize = "SupportsRemoteControl"))]
    pub supports_remote_control: bool,
}


