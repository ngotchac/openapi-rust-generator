use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsersUserPolicy {
    #[serde(rename(deserialize = "IsAdministrator"))]
    pub is_administrator: bool,
    #[serde(rename(deserialize = "IsHidden"))]
    pub is_hidden: bool,
    #[serde(rename(deserialize = "IsHiddenRemotely"))]
    pub is_hidden_remotely: bool,
    #[serde(rename(deserialize = "IsDisabled"))]
    pub is_disabled: bool,
    #[serde(rename(deserialize = "MaxParentalRating"), skip_serializing_if = "Option::is_none")]
    pub max_parental_rating: Option<i32>,
    #[serde(rename(deserialize = "BlockedTags"))]
    pub blocked_tags: Vec<String>,
    #[serde(rename(deserialize = "EnableUserPreferenceAccess"))]
    pub enable_user_preference_access: bool,
    #[serde(rename(deserialize = "AccessSchedules"))]
    pub access_schedules: Vec<Box<crate::models::ConfigurationAccessSchedule>>,
    #[serde(rename(deserialize = "BlockUnratedItems"))]
    pub block_unrated_items: BlockUnratedItems,
    #[serde(rename(deserialize = "EnableRemoteControlOfOtherUsers"))]
    pub enable_remote_control_of_other_users: bool,
    #[serde(rename(deserialize = "EnableSharedDeviceControl"))]
    pub enable_shared_device_control: bool,
    #[serde(rename(deserialize = "EnableRemoteAccess"))]
    pub enable_remote_access: bool,
    #[serde(rename(deserialize = "EnableLiveTvManagement"))]
    pub enable_live_tv_management: bool,
    #[serde(rename(deserialize = "EnableLiveTvAccess"))]
    pub enable_live_tv_access: bool,
    #[serde(rename(deserialize = "EnableMediaPlayback"))]
    pub enable_media_playback: bool,
    #[serde(rename(deserialize = "EnableAudioPlaybackTranscoding"))]
    pub enable_audio_playback_transcoding: bool,
    #[serde(rename(deserialize = "EnableVideoPlaybackTranscoding"))]
    pub enable_video_playback_transcoding: bool,
    #[serde(rename(deserialize = "EnablePlaybackRemuxing"))]
    pub enable_playback_remuxing: bool,
    #[serde(rename(deserialize = "EnableContentDeletion"))]
    pub enable_content_deletion: bool,
    #[serde(rename(deserialize = "EnableContentDeletionFromFolders"))]
    pub enable_content_deletion_from_folders: Vec<String>,
    #[serde(rename(deserialize = "EnableContentDownloading"))]
    pub enable_content_downloading: bool,
    #[serde(rename(deserialize = "EnableSubtitleDownloading"))]
    pub enable_subtitle_downloading: bool,
    #[serde(rename(deserialize = "EnableSubtitleManagement"))]
    pub enable_subtitle_management: bool,
    #[serde(rename(deserialize = "EnableSyncTranscoding"))]
    pub enable_sync_transcoding: bool,
    #[serde(rename(deserialize = "EnableMediaConversion"))]
    pub enable_media_conversion: bool,
    #[serde(rename(deserialize = "EnabledDevices"))]
    pub enabled_devices: Vec<String>,
    #[serde(rename(deserialize = "EnableAllDevices"))]
    pub enable_all_devices: bool,
    #[serde(rename(deserialize = "EnabledChannels"))]
    pub enabled_channels: Vec<String>,
    #[serde(rename(deserialize = "EnableAllChannels"))]
    pub enable_all_channels: bool,
    #[serde(rename(deserialize = "EnabledFolders"))]
    pub enabled_folders: Vec<String>,
    #[serde(rename(deserialize = "EnableAllFolders"))]
    pub enable_all_folders: bool,
    #[serde(rename(deserialize = "InvalidLoginAttemptCount"))]
    pub invalid_login_attempt_count: i32,
    #[serde(rename(deserialize = "EnablePublicSharing"))]
    pub enable_public_sharing: bool,
    #[serde(rename(deserialize = "BlockedMediaFolders"))]
    pub blocked_media_folders: Vec<String>,
    #[serde(rename(deserialize = "BlockedChannels"))]
    pub blocked_channels: Vec<String>,
    #[serde(rename(deserialize = "RemoteClientBitrateLimit"))]
    pub remote_client_bitrate_limit: i32,
    #[serde(rename(deserialize = "AuthenticationProviderId"))]
    pub authentication_provider_id: String,
    #[serde(rename(deserialize = "ExcludedSubFolders"))]
    pub excluded_sub_folders: Vec<String>,
    #[serde(rename(deserialize = "DisablePremiumFeatures"))]
    pub disable_premium_features: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BlockUnratedItems {
    #[serde(rename = "Movie")]
    Movie,
    #[serde(rename = "Trailer")]
    Trailer,
    #[serde(rename = "Series")]
    Series,
    #[serde(rename = "Music")]
    Music,
    #[serde(rename = "Game")]
    Game,
    #[serde(rename = "Book")]
    Book,
    #[serde(rename = "LiveTvChannel")]
    LiveTvChannel,
    #[serde(rename = "LiveTvProgram")]
    LiveTvProgram,
    #[serde(rename = "ChannelContent")]
    ChannelContent,
    #[serde(rename = "Other")]
    Other,
}

