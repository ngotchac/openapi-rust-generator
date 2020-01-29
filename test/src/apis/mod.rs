#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    SerdeJson(serde_path_to_error::Error<serde_json::Error>),
    Io(std::io::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl From<serde_path_to_error::Error<serde_json::Error>> for Error {
    fn from(e: serde_path_to_error::Error<serde_json::Error>) -> Self {
        Error::SerdeJson(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

mod activity_log_service_api;
pub use self::activity_log_service_api::ActivityLogServiceApiClient;
mod artists_service_api;
pub use self::artists_service_api::ArtistsServiceApiClient;
mod audio_service_api;
pub use self::audio_service_api::AudioServiceApiClient;
mod bif_service_api;
pub use self::bif_service_api::BifServiceApiClient;
mod branding_service_api;
pub use self::branding_service_api::BrandingServiceApiClient;
mod channel_service_api;
pub use self::channel_service_api::ChannelServiceApiClient;
mod collection_service_api;
pub use self::collection_service_api::CollectionServiceApiClient;
mod configuration_service_api;
pub use self::configuration_service_api::ConfigurationServiceApiClient;
mod connect_service_api;
pub use self::connect_service_api::ConnectServiceApiClient;
mod dashboard_service_api;
pub use self::dashboard_service_api::DashboardServiceApiClient;
mod device_service_api;
pub use self::device_service_api::DeviceServiceApiClient;
mod display_preferences_service_api;
pub use self::display_preferences_service_api::DisplayPreferencesServiceApiClient;
mod dlna_server_service_api;
pub use self::dlna_server_service_api::DlnaServerServiceApiClient;
mod dlna_service_api;
pub use self::dlna_service_api::DlnaServiceApiClient;
mod dynamic_hls_service_api;
pub use self::dynamic_hls_service_api::DynamicHlsServiceApiClient;
mod encoding_info_service_api;
pub use self::encoding_info_service_api::EncodingInfoServiceApiClient;
mod environment_service_api;
pub use self::environment_service_api::EnvironmentServiceApiClient;
mod filter_service_api;
pub use self::filter_service_api::FilterServiceApiClient;
mod game_genres_service_api;
pub use self::game_genres_service_api::GameGenresServiceApiClient;
mod games_service_api;
pub use self::games_service_api::GamesServiceApiClient;
mod genres_service_api;
pub use self::genres_service_api::GenresServiceApiClient;
mod hls_segment_service_api;
pub use self::hls_segment_service_api::HlsSegmentServiceApiClient;
mod image_by_name_service_api;
pub use self::image_by_name_service_api::ImageByNameServiceApiClient;
mod image_service_api;
pub use self::image_service_api::ImageServiceApiClient;
mod instant_mix_service_api;
pub use self::instant_mix_service_api::InstantMixServiceApiClient;
mod item_lookup_service_api;
pub use self::item_lookup_service_api::ItemLookupServiceApiClient;
mod item_refresh_service_api;
pub use self::item_refresh_service_api::ItemRefreshServiceApiClient;
mod item_update_service_api;
pub use self::item_update_service_api::ItemUpdateServiceApiClient;
mod items_service_api;
pub use self::items_service_api::ItemsServiceApiClient;
mod library_service_api;
pub use self::library_service_api::LibraryServiceApiClient;
mod library_structure_service_api;
pub use self::library_structure_service_api::LibraryStructureServiceApiClient;
mod live_tv_service_api;
pub use self::live_tv_service_api::LiveTvServiceApiClient;
mod localization_service_api;
pub use self::localization_service_api::LocalizationServiceApiClient;
mod media_info_service_api;
pub use self::media_info_service_api::MediaInfoServiceApiClient;
mod movies_service_api;
pub use self::movies_service_api::MoviesServiceApiClient;
mod music_genres_service_api;
pub use self::music_genres_service_api::MusicGenresServiceApiClient;
mod news_service_api;
pub use self::news_service_api::NewsServiceApiClient;
mod notifications_service_api;
pub use self::notifications_service_api::NotificationsServiceApiClient;
mod official_rating_service_api;
pub use self::official_rating_service_api::OfficialRatingServiceApiClient;
mod open_api_service_api;
pub use self::open_api_service_api::OpenApiServiceApiClient;
mod package_service_api;
pub use self::package_service_api::PackageServiceApiClient;
mod persons_service_api;
pub use self::persons_service_api::PersonsServiceApiClient;
mod playlist_service_api;
pub use self::playlist_service_api::PlaylistServiceApiClient;
mod playstate_service_api;
pub use self::playstate_service_api::PlaystateServiceApiClient;
mod plugin_service_api;
pub use self::plugin_service_api::PluginServiceApiClient;
mod remote_image_service_api;
pub use self::remote_image_service_api::RemoteImageServiceApiClient;
mod reports_service_api;
pub use self::reports_service_api::ReportsServiceApiClient;
mod scheduled_task_service_api;
pub use self::scheduled_task_service_api::ScheduledTaskServiceApiClient;
mod search_service_api;
pub use self::search_service_api::SearchServiceApiClient;
mod server_api_endpoints_api;
pub use self::server_api_endpoints_api::ServerApiEndpointsApiClient;
mod sessions_service_api;
pub use self::sessions_service_api::SessionsServiceApiClient;
mod studios_service_api;
pub use self::studios_service_api::StudiosServiceApiClient;
mod subtitle_service_api;
pub use self::subtitle_service_api::SubtitleServiceApiClient;
mod suggestions_service_api;
pub use self::suggestions_service_api::SuggestionsServiceApiClient;
mod sync_service_api;
pub use self::sync_service_api::SyncServiceApiClient;
mod system_service_api;
pub use self::system_service_api::SystemServiceApiClient;
mod tag_service_api;
pub use self::tag_service_api::TagServiceApiClient;
mod trailers_service_api;
pub use self::trailers_service_api::TrailersServiceApiClient;
mod tv_shows_service_api;
pub use self::tv_shows_service_api::TvShowsServiceApiClient;
mod universal_audio_service_api;
pub use self::universal_audio_service_api::UniversalAudioServiceApiClient;
mod user_activity_api_api;
pub use self::user_activity_api_api::UserActivityAPIApiClient;
mod user_library_service_api;
pub use self::user_library_service_api::UserLibraryServiceApiClient;
mod user_service_api;
pub use self::user_service_api::UserServiceApiClient;
mod user_views_service_api;
pub use self::user_views_service_api::UserViewsServiceApiClient;
mod video_hls_service_api;
pub use self::video_hls_service_api::VideoHlsServiceApiClient;
mod video_service_api;
pub use self::video_service_api::VideoServiceApiClient;
mod videos_service_api;
pub use self::videos_service_api::VideosServiceApiClient;

pub mod configuration;
pub mod client;
