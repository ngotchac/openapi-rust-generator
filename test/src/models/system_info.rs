use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemInfo {
    #[serde(rename(deserialize = "SystemUpdateLevel"))]
    pub system_update_level: SystemUpdateLevel,
    #[serde(rename(deserialize = "OperatingSystemDisplayName"))]
    pub operating_system_display_name: String,
    #[serde(rename(deserialize = "PackageName"))]
    pub package_name: String,
    #[serde(rename(deserialize = "HasPendingRestart"))]
    pub has_pending_restart: bool,
    #[serde(rename(deserialize = "IsShuttingDown"))]
    pub is_shutting_down: bool,
    #[serde(rename(deserialize = "SupportsLibraryMonitor"))]
    pub supports_library_monitor: bool,
    #[serde(rename(deserialize = "WebSocketPortNumber"))]
    pub web_socket_port_number: i32,
    #[serde(rename(deserialize = "CompletedInstallations"))]
    pub completed_installations: Vec<Box<crate::models::UpdatesInstallationInfo>>,
    #[serde(rename(deserialize = "CanSelfRestart"))]
    pub can_self_restart: bool,
    #[serde(rename(deserialize = "CanSelfUpdate"))]
    pub can_self_update: bool,
    #[serde(rename(deserialize = "CanLaunchWebBrowser"))]
    pub can_launch_web_browser: bool,
    #[serde(rename(deserialize = "ProgramDataPath"))]
    pub program_data_path: String,
    #[serde(rename(deserialize = "ItemsByNamePath"))]
    pub items_by_name_path: String,
    #[serde(rename(deserialize = "CachePath"))]
    pub cache_path: String,
    #[serde(rename(deserialize = "LogPath"))]
    pub log_path: String,
    #[serde(rename(deserialize = "InternalMetadataPath"))]
    pub internal_metadata_path: String,
    #[serde(rename(deserialize = "TranscodingTempPath"))]
    pub transcoding_temp_path: String,
    #[serde(rename(deserialize = "HttpServerPortNumber"))]
    pub http_server_port_number: i32,
    #[serde(rename(deserialize = "SupportsHttps"))]
    pub supports_https: bool,
    #[serde(rename(deserialize = "HttpsPortNumber"))]
    pub https_port_number: i32,
    #[serde(rename(deserialize = "HasUpdateAvailable"))]
    pub has_update_available: bool,
    #[serde(rename(deserialize = "SupportsAutoRunAtStartup"))]
    pub supports_auto_run_at_startup: bool,
    #[serde(rename(deserialize = "HardwareAccelerationRequiresPremiere"))]
    pub hardware_acceleration_requires_premiere: bool,
    #[serde(rename(deserialize = "LocalAddress"))]
    pub local_address: String,
    #[serde(rename(deserialize = "WanAddress"))]
    pub wan_address: String,
    #[serde(rename(deserialize = "ServerName"))]
    pub server_name: String,
    #[serde(rename(deserialize = "Version"))]
    pub version: String,
    #[serde(rename(deserialize = "OperatingSystem"))]
    pub operating_system: String,
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SystemUpdateLevel {
    #[serde(rename = "Release")]
    Release,
    #[serde(rename = "Beta")]
    Beta,
    #[serde(rename = "Dev")]
    Dev,
}

