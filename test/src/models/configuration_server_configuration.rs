use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationServerConfiguration {
    #[serde(rename(deserialize = "EnableUPnP"))]
    pub enable_u_pn_p: bool,
    #[serde(rename(deserialize = "PublicPort"))]
    pub public_port: i32,
    #[serde(rename(deserialize = "PublicHttpsPort"))]
    pub public_https_port: i32,
    #[serde(rename(deserialize = "HttpServerPortNumber"))]
    pub http_server_port_number: i32,
    #[serde(rename(deserialize = "HttpsPortNumber"))]
    pub https_port_number: i32,
    #[serde(rename(deserialize = "EnableHttps"))]
    pub enable_https: bool,
    #[serde(rename(deserialize = "SubtitlePermissionsUpgraded"))]
    pub subtitle_permissions_upgraded: bool,
    #[serde(rename(deserialize = "CertificatePath"))]
    pub certificate_path: String,
    #[serde(rename(deserialize = "CertificatePassword"))]
    pub certificate_password: String,
    #[serde(rename(deserialize = "IsPortAuthorized"))]
    pub is_port_authorized: bool,
    #[serde(rename(deserialize = "AutoRunWebApp"))]
    pub auto_run_web_app: bool,
    #[serde(rename(deserialize = "EnableRemoteAccess"))]
    pub enable_remote_access: bool,
    #[serde(rename(deserialize = "LogAllQueryTimes"))]
    pub log_all_query_times: bool,
    #[serde(rename(deserialize = "EnableCaseSensitiveItemIds"))]
    pub enable_case_sensitive_item_ids: bool,
    #[serde(rename(deserialize = "MetadataPath"))]
    pub metadata_path: String,
    #[serde(rename(deserialize = "MetadataNetworkPath"))]
    pub metadata_network_path: String,
    #[serde(rename(deserialize = "PreferredMetadataLanguage"))]
    pub preferred_metadata_language: String,
    #[serde(rename(deserialize = "MetadataCountryCode"))]
    pub metadata_country_code: String,
    #[serde(rename(deserialize = "SortReplaceCharacters"))]
    pub sort_replace_characters: Vec<String>,
    #[serde(rename(deserialize = "SortRemoveCharacters"))]
    pub sort_remove_characters: Vec<String>,
    #[serde(rename(deserialize = "SortRemoveWords"))]
    pub sort_remove_words: Vec<String>,
    #[serde(rename(deserialize = "LibraryMonitorDelay"))]
    pub library_monitor_delay: i32,
    #[serde(rename(deserialize = "EnableDashboardResponseCaching"))]
    pub enable_dashboard_response_caching: bool,
    #[serde(rename(deserialize = "DashboardSourcePath"))]
    pub dashboard_source_path: String,
    #[serde(rename(deserialize = "ImageSavingConvention"))]
    pub image_saving_convention: ImageSavingConvention,
    #[serde(rename(deserialize = "EnableAutomaticRestart"))]
    pub enable_automatic_restart: bool,
    #[serde(rename(deserialize = "SkipDeserializationForBasicTypes"))]
    pub skip_deserialization_for_basic_types: bool,
    #[serde(rename(deserialize = "ServerName"))]
    pub server_name: String,
    #[serde(rename(deserialize = "WanDdns"))]
    pub wan_ddns: String,
    #[serde(rename(deserialize = "UICulture"))]
    pub ui_culture: String,
    #[serde(rename(deserialize = "SaveMetadataHidden"))]
    pub save_metadata_hidden: bool,
    #[serde(rename(deserialize = "RemoteClientBitrateLimit"))]
    pub remote_client_bitrate_limit: i32,
    #[serde(rename(deserialize = "SchemaVersion"))]
    pub schema_version: i32,
    #[serde(rename(deserialize = "DisplaySpecialsWithinSeasons"))]
    pub display_specials_within_seasons: bool,
    #[serde(rename(deserialize = "LocalNetworkSubnets"))]
    pub local_network_subnets: Vec<String>,
    #[serde(rename(deserialize = "LocalNetworkAddresses"))]
    pub local_network_addresses: Vec<String>,
    #[serde(rename(deserialize = "EnableExternalContentInSuggestions"))]
    pub enable_external_content_in_suggestions: bool,
    #[serde(rename(deserialize = "RequireHttps"))]
    pub require_https: bool,
    #[serde(rename(deserialize = "IsBehindProxy"))]
    pub is_behind_proxy: bool,
    #[serde(rename(deserialize = "RemoteIPFilter"))]
    pub remote_ip_filter: Vec<String>,
    #[serde(rename(deserialize = "IsRemoteIPFilterBlacklist"))]
    pub is_remote_ip_filter_blacklist: bool,
    #[serde(rename(deserialize = "ImageExtractionTimeoutMs"))]
    pub image_extraction_timeout_ms: i32,
    #[serde(rename(deserialize = "PathSubstitutions"))]
    pub path_substitutions: Vec<Box<crate::models::ConfigurationPathSubstitution>>,
    #[serde(rename(deserialize = "UninstalledPlugins"))]
    pub uninstalled_plugins: Vec<String>,
    #[serde(rename(deserialize = "CollapseVideoFolders"))]
    pub collapse_video_folders: bool,
    #[serde(rename(deserialize = "EnableOriginalTrackTitles"))]
    pub enable_original_track_titles: bool,
    #[serde(rename(deserialize = "EnableDebugLevelLogging"))]
    pub enable_debug_level_logging: bool,
    #[serde(rename(deserialize = "EnableAutoUpdate"))]
    pub enable_auto_update: bool,
    #[serde(rename(deserialize = "LogFileRetentionDays"))]
    pub log_file_retention_days: i32,
    #[serde(rename(deserialize = "RunAtStartup"))]
    pub run_at_startup: bool,
    #[serde(rename(deserialize = "IsStartupWizardCompleted"))]
    pub is_startup_wizard_completed: bool,
    #[serde(rename(deserialize = "CachePath"))]
    pub cache_path: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ImageSavingConvention {
    #[serde(rename = "Legacy")]
    Legacy,
    #[serde(rename = "Compatible")]
    Compatible,
}

