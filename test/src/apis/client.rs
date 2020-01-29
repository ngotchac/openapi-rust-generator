use std::rc::Rc;

use super::configuration::Configuration;

pub struct APIClient {
    activity_log_service_api: crate::apis::ActivityLogServiceApiClient,
    artists_service_api: crate::apis::ArtistsServiceApiClient,
    audio_service_api: crate::apis::AudioServiceApiClient,
    bif_service_api: crate::apis::BifServiceApiClient,
    branding_service_api: crate::apis::BrandingServiceApiClient,
    channel_service_api: crate::apis::ChannelServiceApiClient,
    collection_service_api: crate::apis::CollectionServiceApiClient,
    configuration_service_api: crate::apis::ConfigurationServiceApiClient,
    connect_service_api: crate::apis::ConnectServiceApiClient,
    dashboard_service_api: crate::apis::DashboardServiceApiClient,
    device_service_api: crate::apis::DeviceServiceApiClient,
    display_preferences_service_api: crate::apis::DisplayPreferencesServiceApiClient,
    dlna_server_service_api: crate::apis::DlnaServerServiceApiClient,
    dlna_service_api: crate::apis::DlnaServiceApiClient,
    dynamic_hls_service_api: crate::apis::DynamicHlsServiceApiClient,
    encoding_info_service_api: crate::apis::EncodingInfoServiceApiClient,
    environment_service_api: crate::apis::EnvironmentServiceApiClient,
    filter_service_api: crate::apis::FilterServiceApiClient,
    game_genres_service_api: crate::apis::GameGenresServiceApiClient,
    games_service_api: crate::apis::GamesServiceApiClient,
    genres_service_api: crate::apis::GenresServiceApiClient,
    hls_segment_service_api: crate::apis::HlsSegmentServiceApiClient,
    image_by_name_service_api: crate::apis::ImageByNameServiceApiClient,
    image_service_api: crate::apis::ImageServiceApiClient,
    instant_mix_service_api: crate::apis::InstantMixServiceApiClient,
    item_lookup_service_api: crate::apis::ItemLookupServiceApiClient,
    item_refresh_service_api: crate::apis::ItemRefreshServiceApiClient,
    item_update_service_api: crate::apis::ItemUpdateServiceApiClient,
    items_service_api: crate::apis::ItemsServiceApiClient,
    library_service_api: crate::apis::LibraryServiceApiClient,
    library_structure_service_api: crate::apis::LibraryStructureServiceApiClient,
    live_tv_service_api: crate::apis::LiveTvServiceApiClient,
    localization_service_api: crate::apis::LocalizationServiceApiClient,
    media_info_service_api: crate::apis::MediaInfoServiceApiClient,
    movies_service_api: crate::apis::MoviesServiceApiClient,
    music_genres_service_api: crate::apis::MusicGenresServiceApiClient,
    news_service_api: crate::apis::NewsServiceApiClient,
    notifications_service_api: crate::apis::NotificationsServiceApiClient,
    official_rating_service_api: crate::apis::OfficialRatingServiceApiClient,
    open_api_service_api: crate::apis::OpenApiServiceApiClient,
    package_service_api: crate::apis::PackageServiceApiClient,
    persons_service_api: crate::apis::PersonsServiceApiClient,
    playlist_service_api: crate::apis::PlaylistServiceApiClient,
    playstate_service_api: crate::apis::PlaystateServiceApiClient,
    plugin_service_api: crate::apis::PluginServiceApiClient,
    remote_image_service_api: crate::apis::RemoteImageServiceApiClient,
    reports_service_api: crate::apis::ReportsServiceApiClient,
    scheduled_task_service_api: crate::apis::ScheduledTaskServiceApiClient,
    search_service_api: crate::apis::SearchServiceApiClient,
    server_api_endpoints_api: crate::apis::ServerApiEndpointsApiClient,
    sessions_service_api: crate::apis::SessionsServiceApiClient,
    studios_service_api: crate::apis::StudiosServiceApiClient,
    subtitle_service_api: crate::apis::SubtitleServiceApiClient,
    suggestions_service_api: crate::apis::SuggestionsServiceApiClient,
    sync_service_api: crate::apis::SyncServiceApiClient,
    system_service_api: crate::apis::SystemServiceApiClient,
    tag_service_api: crate::apis::TagServiceApiClient,
    trailers_service_api: crate::apis::TrailersServiceApiClient,
    tv_shows_service_api: crate::apis::TvShowsServiceApiClient,
    universal_audio_service_api: crate::apis::UniversalAudioServiceApiClient,
    user_activity_api_api: crate::apis::UserActivityAPIApiClient,
    user_library_service_api: crate::apis::UserLibraryServiceApiClient,
    user_service_api: crate::apis::UserServiceApiClient,
    user_views_service_api: crate::apis::UserViewsServiceApiClient,
    video_hls_service_api: crate::apis::VideoHlsServiceApiClient,
    video_service_api: crate::apis::VideoServiceApiClient,
    videos_service_api: crate::apis::VideosServiceApiClient,
}

impl APIClient {
    pub fn new(configuration: Configuration) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            activity_log_service_api: crate::apis::ActivityLogServiceApiClient::new(rc.clone()),
            artists_service_api: crate::apis::ArtistsServiceApiClient::new(rc.clone()),
            audio_service_api: crate::apis::AudioServiceApiClient::new(rc.clone()),
            bif_service_api: crate::apis::BifServiceApiClient::new(rc.clone()),
            branding_service_api: crate::apis::BrandingServiceApiClient::new(rc.clone()),
            channel_service_api: crate::apis::ChannelServiceApiClient::new(rc.clone()),
            collection_service_api: crate::apis::CollectionServiceApiClient::new(rc.clone()),
            configuration_service_api: crate::apis::ConfigurationServiceApiClient::new(rc.clone()),
            connect_service_api: crate::apis::ConnectServiceApiClient::new(rc.clone()),
            dashboard_service_api: crate::apis::DashboardServiceApiClient::new(rc.clone()),
            device_service_api: crate::apis::DeviceServiceApiClient::new(rc.clone()),
            display_preferences_service_api: crate::apis::DisplayPreferencesServiceApiClient::new(rc.clone()),
            dlna_server_service_api: crate::apis::DlnaServerServiceApiClient::new(rc.clone()),
            dlna_service_api: crate::apis::DlnaServiceApiClient::new(rc.clone()),
            dynamic_hls_service_api: crate::apis::DynamicHlsServiceApiClient::new(rc.clone()),
            encoding_info_service_api: crate::apis::EncodingInfoServiceApiClient::new(rc.clone()),
            environment_service_api: crate::apis::EnvironmentServiceApiClient::new(rc.clone()),
            filter_service_api: crate::apis::FilterServiceApiClient::new(rc.clone()),
            game_genres_service_api: crate::apis::GameGenresServiceApiClient::new(rc.clone()),
            games_service_api: crate::apis::GamesServiceApiClient::new(rc.clone()),
            genres_service_api: crate::apis::GenresServiceApiClient::new(rc.clone()),
            hls_segment_service_api: crate::apis::HlsSegmentServiceApiClient::new(rc.clone()),
            image_by_name_service_api: crate::apis::ImageByNameServiceApiClient::new(rc.clone()),
            image_service_api: crate::apis::ImageServiceApiClient::new(rc.clone()),
            instant_mix_service_api: crate::apis::InstantMixServiceApiClient::new(rc.clone()),
            item_lookup_service_api: crate::apis::ItemLookupServiceApiClient::new(rc.clone()),
            item_refresh_service_api: crate::apis::ItemRefreshServiceApiClient::new(rc.clone()),
            item_update_service_api: crate::apis::ItemUpdateServiceApiClient::new(rc.clone()),
            items_service_api: crate::apis::ItemsServiceApiClient::new(rc.clone()),
            library_service_api: crate::apis::LibraryServiceApiClient::new(rc.clone()),
            library_structure_service_api: crate::apis::LibraryStructureServiceApiClient::new(rc.clone()),
            live_tv_service_api: crate::apis::LiveTvServiceApiClient::new(rc.clone()),
            localization_service_api: crate::apis::LocalizationServiceApiClient::new(rc.clone()),
            media_info_service_api: crate::apis::MediaInfoServiceApiClient::new(rc.clone()),
            movies_service_api: crate::apis::MoviesServiceApiClient::new(rc.clone()),
            music_genres_service_api: crate::apis::MusicGenresServiceApiClient::new(rc.clone()),
            news_service_api: crate::apis::NewsServiceApiClient::new(rc.clone()),
            notifications_service_api: crate::apis::NotificationsServiceApiClient::new(rc.clone()),
            official_rating_service_api: crate::apis::OfficialRatingServiceApiClient::new(rc.clone()),
            open_api_service_api: crate::apis::OpenApiServiceApiClient::new(rc.clone()),
            package_service_api: crate::apis::PackageServiceApiClient::new(rc.clone()),
            persons_service_api: crate::apis::PersonsServiceApiClient::new(rc.clone()),
            playlist_service_api: crate::apis::PlaylistServiceApiClient::new(rc.clone()),
            playstate_service_api: crate::apis::PlaystateServiceApiClient::new(rc.clone()),
            plugin_service_api: crate::apis::PluginServiceApiClient::new(rc.clone()),
            remote_image_service_api: crate::apis::RemoteImageServiceApiClient::new(rc.clone()),
            reports_service_api: crate::apis::ReportsServiceApiClient::new(rc.clone()),
            scheduled_task_service_api: crate::apis::ScheduledTaskServiceApiClient::new(rc.clone()),
            search_service_api: crate::apis::SearchServiceApiClient::new(rc.clone()),
            server_api_endpoints_api: crate::apis::ServerApiEndpointsApiClient::new(rc.clone()),
            sessions_service_api: crate::apis::SessionsServiceApiClient::new(rc.clone()),
            studios_service_api: crate::apis::StudiosServiceApiClient::new(rc.clone()),
            subtitle_service_api: crate::apis::SubtitleServiceApiClient::new(rc.clone()),
            suggestions_service_api: crate::apis::SuggestionsServiceApiClient::new(rc.clone()),
            sync_service_api: crate::apis::SyncServiceApiClient::new(rc.clone()),
            system_service_api: crate::apis::SystemServiceApiClient::new(rc.clone()),
            tag_service_api: crate::apis::TagServiceApiClient::new(rc.clone()),
            trailers_service_api: crate::apis::TrailersServiceApiClient::new(rc.clone()),
            tv_shows_service_api: crate::apis::TvShowsServiceApiClient::new(rc.clone()),
            universal_audio_service_api: crate::apis::UniversalAudioServiceApiClient::new(rc.clone()),
            user_activity_api_api: crate::apis::UserActivityAPIApiClient::new(rc.clone()),
            user_library_service_api: crate::apis::UserLibraryServiceApiClient::new(rc.clone()),
            user_service_api: crate::apis::UserServiceApiClient::new(rc.clone()),
            user_views_service_api: crate::apis::UserViewsServiceApiClient::new(rc.clone()),
            video_hls_service_api: crate::apis::VideoHlsServiceApiClient::new(rc.clone()),
            video_service_api: crate::apis::VideoServiceApiClient::new(rc.clone()),
            videos_service_api: crate::apis::VideosServiceApiClient::new(rc.clone()),
        }
    }

    pub fn activity_log_service_api(&self) -> &crate::apis::ActivityLogServiceApiClient {
        &self.activity_log_service_api
    }

    pub fn artists_service_api(&self) -> &crate::apis::ArtistsServiceApiClient {
        &self.artists_service_api
    }

    pub fn audio_service_api(&self) -> &crate::apis::AudioServiceApiClient {
        &self.audio_service_api
    }

    pub fn bif_service_api(&self) -> &crate::apis::BifServiceApiClient {
        &self.bif_service_api
    }

    pub fn branding_service_api(&self) -> &crate::apis::BrandingServiceApiClient {
        &self.branding_service_api
    }

    pub fn channel_service_api(&self) -> &crate::apis::ChannelServiceApiClient {
        &self.channel_service_api
    }

    pub fn collection_service_api(&self) -> &crate::apis::CollectionServiceApiClient {
        &self.collection_service_api
    }

    pub fn configuration_service_api(&self) -> &crate::apis::ConfigurationServiceApiClient {
        &self.configuration_service_api
    }

    pub fn connect_service_api(&self) -> &crate::apis::ConnectServiceApiClient {
        &self.connect_service_api
    }

    pub fn dashboard_service_api(&self) -> &crate::apis::DashboardServiceApiClient {
        &self.dashboard_service_api
    }

    pub fn device_service_api(&self) -> &crate::apis::DeviceServiceApiClient {
        &self.device_service_api
    }

    pub fn display_preferences_service_api(&self) -> &crate::apis::DisplayPreferencesServiceApiClient {
        &self.display_preferences_service_api
    }

    pub fn dlna_server_service_api(&self) -> &crate::apis::DlnaServerServiceApiClient {
        &self.dlna_server_service_api
    }

    pub fn dlna_service_api(&self) -> &crate::apis::DlnaServiceApiClient {
        &self.dlna_service_api
    }

    pub fn dynamic_hls_service_api(&self) -> &crate::apis::DynamicHlsServiceApiClient {
        &self.dynamic_hls_service_api
    }

    pub fn encoding_info_service_api(&self) -> &crate::apis::EncodingInfoServiceApiClient {
        &self.encoding_info_service_api
    }

    pub fn environment_service_api(&self) -> &crate::apis::EnvironmentServiceApiClient {
        &self.environment_service_api
    }

    pub fn filter_service_api(&self) -> &crate::apis::FilterServiceApiClient {
        &self.filter_service_api
    }

    pub fn game_genres_service_api(&self) -> &crate::apis::GameGenresServiceApiClient {
        &self.game_genres_service_api
    }

    pub fn games_service_api(&self) -> &crate::apis::GamesServiceApiClient {
        &self.games_service_api
    }

    pub fn genres_service_api(&self) -> &crate::apis::GenresServiceApiClient {
        &self.genres_service_api
    }

    pub fn hls_segment_service_api(&self) -> &crate::apis::HlsSegmentServiceApiClient {
        &self.hls_segment_service_api
    }

    pub fn image_by_name_service_api(&self) -> &crate::apis::ImageByNameServiceApiClient {
        &self.image_by_name_service_api
    }

    pub fn image_service_api(&self) -> &crate::apis::ImageServiceApiClient {
        &self.image_service_api
    }

    pub fn instant_mix_service_api(&self) -> &crate::apis::InstantMixServiceApiClient {
        &self.instant_mix_service_api
    }

    pub fn item_lookup_service_api(&self) -> &crate::apis::ItemLookupServiceApiClient {
        &self.item_lookup_service_api
    }

    pub fn item_refresh_service_api(&self) -> &crate::apis::ItemRefreshServiceApiClient {
        &self.item_refresh_service_api
    }

    pub fn item_update_service_api(&self) -> &crate::apis::ItemUpdateServiceApiClient {
        &self.item_update_service_api
    }

    pub fn items_service_api(&self) -> &crate::apis::ItemsServiceApiClient {
        &self.items_service_api
    }

    pub fn library_service_api(&self) -> &crate::apis::LibraryServiceApiClient {
        &self.library_service_api
    }

    pub fn library_structure_service_api(&self) -> &crate::apis::LibraryStructureServiceApiClient {
        &self.library_structure_service_api
    }

    pub fn live_tv_service_api(&self) -> &crate::apis::LiveTvServiceApiClient {
        &self.live_tv_service_api
    }

    pub fn localization_service_api(&self) -> &crate::apis::LocalizationServiceApiClient {
        &self.localization_service_api
    }

    pub fn media_info_service_api(&self) -> &crate::apis::MediaInfoServiceApiClient {
        &self.media_info_service_api
    }

    pub fn movies_service_api(&self) -> &crate::apis::MoviesServiceApiClient {
        &self.movies_service_api
    }

    pub fn music_genres_service_api(&self) -> &crate::apis::MusicGenresServiceApiClient {
        &self.music_genres_service_api
    }

    pub fn news_service_api(&self) -> &crate::apis::NewsServiceApiClient {
        &self.news_service_api
    }

    pub fn notifications_service_api(&self) -> &crate::apis::NotificationsServiceApiClient {
        &self.notifications_service_api
    }

    pub fn official_rating_service_api(&self) -> &crate::apis::OfficialRatingServiceApiClient {
        &self.official_rating_service_api
    }

    pub fn open_api_service_api(&self) -> &crate::apis::OpenApiServiceApiClient {
        &self.open_api_service_api
    }

    pub fn package_service_api(&self) -> &crate::apis::PackageServiceApiClient {
        &self.package_service_api
    }

    pub fn persons_service_api(&self) -> &crate::apis::PersonsServiceApiClient {
        &self.persons_service_api
    }

    pub fn playlist_service_api(&self) -> &crate::apis::PlaylistServiceApiClient {
        &self.playlist_service_api
    }

    pub fn playstate_service_api(&self) -> &crate::apis::PlaystateServiceApiClient {
        &self.playstate_service_api
    }

    pub fn plugin_service_api(&self) -> &crate::apis::PluginServiceApiClient {
        &self.plugin_service_api
    }

    pub fn remote_image_service_api(&self) -> &crate::apis::RemoteImageServiceApiClient {
        &self.remote_image_service_api
    }

    pub fn reports_service_api(&self) -> &crate::apis::ReportsServiceApiClient {
        &self.reports_service_api
    }

    pub fn scheduled_task_service_api(&self) -> &crate::apis::ScheduledTaskServiceApiClient {
        &self.scheduled_task_service_api
    }

    pub fn search_service_api(&self) -> &crate::apis::SearchServiceApiClient {
        &self.search_service_api
    }

    pub fn server_api_endpoints_api(&self) -> &crate::apis::ServerApiEndpointsApiClient {
        &self.server_api_endpoints_api
    }

    pub fn sessions_service_api(&self) -> &crate::apis::SessionsServiceApiClient {
        &self.sessions_service_api
    }

    pub fn studios_service_api(&self) -> &crate::apis::StudiosServiceApiClient {
        &self.studios_service_api
    }

    pub fn subtitle_service_api(&self) -> &crate::apis::SubtitleServiceApiClient {
        &self.subtitle_service_api
    }

    pub fn suggestions_service_api(&self) -> &crate::apis::SuggestionsServiceApiClient {
        &self.suggestions_service_api
    }

    pub fn sync_service_api(&self) -> &crate::apis::SyncServiceApiClient {
        &self.sync_service_api
    }

    pub fn system_service_api(&self) -> &crate::apis::SystemServiceApiClient {
        &self.system_service_api
    }

    pub fn tag_service_api(&self) -> &crate::apis::TagServiceApiClient {
        &self.tag_service_api
    }

    pub fn trailers_service_api(&self) -> &crate::apis::TrailersServiceApiClient {
        &self.trailers_service_api
    }

    pub fn tv_shows_service_api(&self) -> &crate::apis::TvShowsServiceApiClient {
        &self.tv_shows_service_api
    }

    pub fn universal_audio_service_api(&self) -> &crate::apis::UniversalAudioServiceApiClient {
        &self.universal_audio_service_api
    }

    pub fn user_activity_api_api(&self) -> &crate::apis::UserActivityAPIApiClient {
        &self.user_activity_api_api
    }

    pub fn user_library_service_api(&self) -> &crate::apis::UserLibraryServiceApiClient {
        &self.user_library_service_api
    }

    pub fn user_service_api(&self) -> &crate::apis::UserServiceApiClient {
        &self.user_service_api
    }

    pub fn user_views_service_api(&self) -> &crate::apis::UserViewsServiceApiClient {
        &self.user_views_service_api
    }

    pub fn video_hls_service_api(&self) -> &crate::apis::VideoHlsServiceApiClient {
        &self.video_hls_service_api
    }

    pub fn video_service_api(&self) -> &crate::apis::VideoServiceApiClient {
        &self.video_service_api
    }

    pub fn videos_service_api(&self) -> &crate::apis::VideosServiceApiClient {
        &self.videos_service_api
    }

}
