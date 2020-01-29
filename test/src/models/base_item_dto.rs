use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseItemDto {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "OriginalTitle"))]
    pub original_title: String,
    #[serde(rename(deserialize = "ServerId"))]
    pub server_id: String,
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
    #[serde(rename(deserialize = "Etag"))]
    pub etag: String,
    #[serde(rename(deserialize = "PlaylistItemId"))]
    pub playlist_item_id: String,
    #[serde(rename(deserialize = "DateCreated"), skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(rename(deserialize = "ExtraType"))]
    pub extra_type: String,
    #[serde(rename(deserialize = "AirsBeforeSeasonNumber"), skip_serializing_if = "Option::is_none")]
    pub airs_before_season_number: Option<i32>,
    #[serde(rename(deserialize = "AirsAfterSeasonNumber"), skip_serializing_if = "Option::is_none")]
    pub airs_after_season_number: Option<i32>,
    #[serde(rename(deserialize = "AirsBeforeEpisodeNumber"), skip_serializing_if = "Option::is_none")]
    pub airs_before_episode_number: Option<i32>,
    #[serde(rename(deserialize = "DisplaySpecialsWithSeasons"), skip_serializing_if = "Option::is_none")]
    pub display_specials_with_seasons: Option<bool>,
    #[serde(rename(deserialize = "CanDelete"), skip_serializing_if = "Option::is_none")]
    pub can_delete: Option<bool>,
    #[serde(rename(deserialize = "CanDownload"), skip_serializing_if = "Option::is_none")]
    pub can_download: Option<bool>,
    #[serde(rename(deserialize = "HasSubtitles"), skip_serializing_if = "Option::is_none")]
    pub has_subtitles: Option<bool>,
    #[serde(rename(deserialize = "SupportsResume"), skip_serializing_if = "Option::is_none")]
    pub supports_resume: Option<bool>,
    #[serde(rename(deserialize = "PreferredMetadataLanguage"))]
    pub preferred_metadata_language: String,
    #[serde(rename(deserialize = "PreferredMetadataCountryCode"))]
    pub preferred_metadata_country_code: String,
    #[serde(rename(deserialize = "SupportsSync"), skip_serializing_if = "Option::is_none")]
    pub supports_sync: Option<bool>,
    #[serde(rename(deserialize = "Container"))]
    pub container: String,
    #[serde(rename(deserialize = "SortName"))]
    pub sort_name: String,
    #[serde(rename(deserialize = "ForcedSortName"))]
    pub forced_sort_name: String,
    #[serde(rename(deserialize = "Video3DFormat"))]
    pub video3_d_format: Video3DFormat,
    #[serde(rename(deserialize = "PremiereDate"), skip_serializing_if = "Option::is_none")]
    pub premiere_date: Option<String>,
    #[serde(rename(deserialize = "ExternalUrls"))]
    pub external_urls: Vec<Box<crate::models::ExternalUrl>>,
    #[serde(rename(deserialize = "MediaSources"))]
    pub media_sources: Vec<Box<crate::models::MediaSourceInfo>>,
    #[serde(rename(deserialize = "CriticRating"), skip_serializing_if = "Option::is_none")]
    pub critic_rating: Option<f32>,
    #[serde(rename(deserialize = "GameSystemId"), skip_serializing_if = "Option::is_none")]
    pub game_system_id: Option<i64>,
    #[serde(rename(deserialize = "GameSystem"))]
    pub game_system: String,
    #[serde(rename(deserialize = "ProductionLocations"))]
    pub production_locations: Vec<String>,
    #[serde(rename(deserialize = "Path"))]
    pub path: String,
    #[serde(rename(deserialize = "OfficialRating"))]
    pub official_rating: String,
    #[serde(rename(deserialize = "CustomRating"))]
    pub custom_rating: String,
    #[serde(rename(deserialize = "ChannelId"))]
    pub channel_id: String,
    #[serde(rename(deserialize = "ChannelName"))]
    pub channel_name: String,
    #[serde(rename(deserialize = "Overview"))]
    pub overview: String,
    #[serde(rename(deserialize = "Taglines"))]
    pub taglines: Vec<String>,
    #[serde(rename(deserialize = "Genres"))]
    pub genres: Vec<String>,
    #[serde(rename(deserialize = "CommunityRating"), skip_serializing_if = "Option::is_none")]
    pub community_rating: Option<f32>,
    #[serde(rename(deserialize = "RunTimeTicks"), skip_serializing_if = "Option::is_none")]
    pub run_time_ticks: Option<i64>,
    #[serde(rename(deserialize = "PlayAccess"))]
    pub play_access: PlayAccess,
    #[serde(rename(deserialize = "AspectRatio"))]
    pub aspect_ratio: String,
    #[serde(rename(deserialize = "ProductionYear"), skip_serializing_if = "Option::is_none")]
    pub production_year: Option<i32>,
    #[serde(rename(deserialize = "Number"))]
    pub number: String,
    #[serde(rename(deserialize = "ChannelNumber"))]
    pub channel_number: String,
    #[serde(rename(deserialize = "IndexNumber"), skip_serializing_if = "Option::is_none")]
    pub index_number: Option<i32>,
    #[serde(rename(deserialize = "IndexNumberEnd"), skip_serializing_if = "Option::is_none")]
    pub index_number_end: Option<i32>,
    #[serde(rename(deserialize = "ParentIndexNumber"), skip_serializing_if = "Option::is_none")]
    pub parent_index_number: Option<i32>,
    #[serde(rename(deserialize = "RemoteTrailers"))]
    pub remote_trailers: Vec<Box<crate::models::MediaUrl>>,
    #[serde(rename(deserialize = "ProviderIds"))]
    pub provider_ids: ::std::collections::HashMap<String, String>,
    #[serde(rename(deserialize = "IsFolder"), skip_serializing_if = "Option::is_none")]
    pub is_folder: Option<bool>,
    #[serde(rename(deserialize = "ParentId"))]
    pub parent_id: String,
    #[serde(rename(deserialize = "Type"))]
    pub _type: String,
    #[serde(rename(deserialize = "People"))]
    pub people: Vec<Box<crate::models::BaseItemPerson>>,
    #[serde(rename(deserialize = "Studios"))]
    pub studios: Vec<Box<crate::models::NameLongIdPair>>,
    #[serde(rename(deserialize = "GenreItems"))]
    pub genre_items: Vec<Box<crate::models::NameLongIdPair>>,
    #[serde(rename(deserialize = "ParentLogoItemId"))]
    pub parent_logo_item_id: String,
    #[serde(rename(deserialize = "ParentBackdropItemId"))]
    pub parent_backdrop_item_id: String,
    #[serde(rename(deserialize = "ParentBackdropImageTags"))]
    pub parent_backdrop_image_tags: Vec<String>,
    #[serde(rename(deserialize = "LocalTrailerCount"), skip_serializing_if = "Option::is_none")]
    pub local_trailer_count: Option<i32>,
    #[serde(rename(deserialize = "UserData"))]
    pub user_data: Box<crate::models::UserItemDataDto>,
    #[serde(rename(deserialize = "RecursiveItemCount"), skip_serializing_if = "Option::is_none")]
    pub recursive_item_count: Option<i32>,
    #[serde(rename(deserialize = "ChildCount"), skip_serializing_if = "Option::is_none")]
    pub child_count: Option<i32>,
    #[serde(rename(deserialize = "SeriesName"))]
    pub series_name: String,
    #[serde(rename(deserialize = "SeriesId"))]
    pub series_id: String,
    #[serde(rename(deserialize = "SeasonId"))]
    pub season_id: String,
    #[serde(rename(deserialize = "SpecialFeatureCount"), skip_serializing_if = "Option::is_none")]
    pub special_feature_count: Option<i32>,
    #[serde(rename(deserialize = "DisplayPreferencesId"))]
    pub display_preferences_id: String,
    #[serde(rename(deserialize = "Status"))]
    pub status: String,
    #[serde(rename(deserialize = "AirTime"))]
    pub air_time: String,
    #[serde(rename(deserialize = "AirDays"))]
    pub air_days: AirDays,
    #[serde(rename(deserialize = "Tags"))]
    pub tags: Vec<String>,
    #[serde(rename(deserialize = "PrimaryImageAspectRatio"), skip_serializing_if = "Option::is_none")]
    pub primary_image_aspect_ratio: Option<f64>,
    #[serde(rename(deserialize = "Artists"))]
    pub artists: Vec<String>,
    #[serde(rename(deserialize = "ArtistItems"))]
    pub artist_items: Vec<Box<crate::models::NameIdPair>>,
    #[serde(rename(deserialize = "Album"))]
    pub album: String,
    #[serde(rename(deserialize = "CollectionType"))]
    pub collection_type: String,
    #[serde(rename(deserialize = "DisplayOrder"))]
    pub display_order: String,
    #[serde(rename(deserialize = "AlbumId"))]
    pub album_id: String,
    #[serde(rename(deserialize = "AlbumPrimaryImageTag"))]
    pub album_primary_image_tag: String,
    #[serde(rename(deserialize = "SeriesPrimaryImageTag"))]
    pub series_primary_image_tag: String,
    #[serde(rename(deserialize = "AlbumArtist"))]
    pub album_artist: String,
    #[serde(rename(deserialize = "AlbumArtists"))]
    pub album_artists: Vec<Box<crate::models::NameIdPair>>,
    #[serde(rename(deserialize = "SeasonName"))]
    pub season_name: String,
    #[serde(rename(deserialize = "MediaStreams"))]
    pub media_streams: Vec<Box<crate::models::MediaStream>>,
    #[serde(rename(deserialize = "PartCount"), skip_serializing_if = "Option::is_none")]
    pub part_count: Option<i32>,
    #[serde(rename(deserialize = "ImageTags"))]
    pub image_tags: ::std::collections::HashMap<String, String>,
    #[serde(rename(deserialize = "BackdropImageTags"))]
    pub backdrop_image_tags: Vec<String>,
    #[serde(rename(deserialize = "ParentLogoImageTag"))]
    pub parent_logo_image_tag: String,
    #[serde(rename(deserialize = "ParentArtItemId"))]
    pub parent_art_item_id: String,
    #[serde(rename(deserialize = "ParentArtImageTag"))]
    pub parent_art_image_tag: String,
    #[serde(rename(deserialize = "SeriesThumbImageTag"))]
    pub series_thumb_image_tag: String,
    #[serde(rename(deserialize = "SeriesStudio"))]
    pub series_studio: String,
    #[serde(rename(deserialize = "ParentThumbItemId"))]
    pub parent_thumb_item_id: String,
    #[serde(rename(deserialize = "ParentThumbImageTag"))]
    pub parent_thumb_image_tag: String,
    #[serde(rename(deserialize = "ParentPrimaryImageItemId"))]
    pub parent_primary_image_item_id: String,
    #[serde(rename(deserialize = "ParentPrimaryImageTag"))]
    pub parent_primary_image_tag: String,
    #[serde(rename(deserialize = "Chapters"))]
    pub chapters: Vec<Box<crate::models::ChapterInfo>>,
    #[serde(rename(deserialize = "LocationType"))]
    pub location_type: LocationType,
    #[serde(rename(deserialize = "MediaType"))]
    pub media_type: String,
    #[serde(rename(deserialize = "EndDate"), skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename(deserialize = "LockedFields"))]
    pub locked_fields: LockedFields,
    #[serde(rename(deserialize = "LockData"), skip_serializing_if = "Option::is_none")]
    pub lock_data: Option<bool>,
    #[serde(rename(deserialize = "Width"), skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(rename(deserialize = "Height"), skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(rename(deserialize = "CameraMake"))]
    pub camera_make: String,
    #[serde(rename(deserialize = "CameraModel"))]
    pub camera_model: String,
    #[serde(rename(deserialize = "Software"))]
    pub software: String,
    #[serde(rename(deserialize = "ExposureTime"), skip_serializing_if = "Option::is_none")]
    pub exposure_time: Option<f64>,
    #[serde(rename(deserialize = "FocalLength"), skip_serializing_if = "Option::is_none")]
    pub focal_length: Option<f64>,
    #[serde(rename(deserialize = "ImageOrientation"))]
    pub image_orientation: ImageOrientation,
    #[serde(rename(deserialize = "Aperture"), skip_serializing_if = "Option::is_none")]
    pub aperture: Option<f64>,
    #[serde(rename(deserialize = "ShutterSpeed"), skip_serializing_if = "Option::is_none")]
    pub shutter_speed: Option<f64>,
    #[serde(rename(deserialize = "Latitude"), skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(rename(deserialize = "Longitude"), skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(rename(deserialize = "Altitude"), skip_serializing_if = "Option::is_none")]
    pub altitude: Option<f64>,
    #[serde(rename(deserialize = "IsoSpeedRating"), skip_serializing_if = "Option::is_none")]
    pub iso_speed_rating: Option<i32>,
    #[serde(rename(deserialize = "SeriesTimerId"))]
    pub series_timer_id: String,
    #[serde(rename(deserialize = "ChannelPrimaryImageTag"))]
    pub channel_primary_image_tag: String,
    #[serde(rename(deserialize = "StartDate"), skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename(deserialize = "CompletionPercentage"), skip_serializing_if = "Option::is_none")]
    pub completion_percentage: Option<f64>,
    #[serde(rename(deserialize = "IsRepeat"), skip_serializing_if = "Option::is_none")]
    pub is_repeat: Option<bool>,
    #[serde(rename(deserialize = "IsNew"), skip_serializing_if = "Option::is_none")]
    pub is_new: Option<bool>,
    #[serde(rename(deserialize = "EpisodeTitle"))]
    pub episode_title: String,
    #[serde(rename(deserialize = "IsMovie"), skip_serializing_if = "Option::is_none")]
    pub is_movie: Option<bool>,
    #[serde(rename(deserialize = "IsSports"), skip_serializing_if = "Option::is_none")]
    pub is_sports: Option<bool>,
    #[serde(rename(deserialize = "IsSeries"), skip_serializing_if = "Option::is_none")]
    pub is_series: Option<bool>,
    #[serde(rename(deserialize = "IsLive"), skip_serializing_if = "Option::is_none")]
    pub is_live: Option<bool>,
    #[serde(rename(deserialize = "IsNews"), skip_serializing_if = "Option::is_none")]
    pub is_news: Option<bool>,
    #[serde(rename(deserialize = "IsKids"), skip_serializing_if = "Option::is_none")]
    pub is_kids: Option<bool>,
    #[serde(rename(deserialize = "IsPremiere"), skip_serializing_if = "Option::is_none")]
    pub is_premiere: Option<bool>,
    #[serde(rename(deserialize = "TimerId"))]
    pub timer_id: String,
    #[serde(rename(deserialize = "CurrentProgram"))]
    pub current_program: Box<crate::models::BaseItemDto>,
    #[serde(rename(deserialize = "MovieCount"), skip_serializing_if = "Option::is_none")]
    pub movie_count: Option<i32>,
    #[serde(rename(deserialize = "SeriesCount"), skip_serializing_if = "Option::is_none")]
    pub series_count: Option<i32>,
    #[serde(rename(deserialize = "AlbumCount"), skip_serializing_if = "Option::is_none")]
    pub album_count: Option<i32>,
    #[serde(rename(deserialize = "SongCount"), skip_serializing_if = "Option::is_none")]
    pub song_count: Option<i32>,
    #[serde(rename(deserialize = "MusicVideoCount"), skip_serializing_if = "Option::is_none")]
    pub music_video_count: Option<i32>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Video3DFormat {
    #[serde(rename = "HalfSideBySide")]
    HalfSideBySide,
    #[serde(rename = "FullSideBySide")]
    FullSideBySide,
    #[serde(rename = "FullTopAndBottom")]
    FullTopAndBottom,
    #[serde(rename = "HalfTopAndBottom")]
    HalfTopAndBottom,
    #[serde(rename = "MVC")]
    MVC,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlayAccess {
    #[serde(rename = "Full")]
    Full,
    #[serde(rename = "None")]
    None,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AirDays {
    #[serde(rename = "Sunday")]
    Sunday,
    #[serde(rename = "Monday")]
    Monday,
    #[serde(rename = "Tuesday")]
    Tuesday,
    #[serde(rename = "Wednesday")]
    Wednesday,
    #[serde(rename = "Thursday")]
    Thursday,
    #[serde(rename = "Friday")]
    Friday,
    #[serde(rename = "Saturday")]
    Saturday,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LocationType {
    #[serde(rename = "FileSystem")]
    FileSystem,
    #[serde(rename = "Virtual")]
    _Virtual,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LockedFields {
    #[serde(rename = "Cast")]
    Cast,
    #[serde(rename = "Genres")]
    Genres,
    #[serde(rename = "ProductionLocations")]
    ProductionLocations,
    #[serde(rename = "Studios")]
    Studios,
    #[serde(rename = "Tags")]
    Tags,
    #[serde(rename = "Name")]
    Name,
    #[serde(rename = "Overview")]
    Overview,
    #[serde(rename = "Runtime")]
    Runtime,
    #[serde(rename = "OfficialRating")]
    OfficialRating,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ImageOrientation {
    #[serde(rename = "TopLeft")]
    TopLeft,
    #[serde(rename = "TopRight")]
    TopRight,
    #[serde(rename = "BottomRight")]
    BottomRight,
    #[serde(rename = "BottomLeft")]
    BottomLeft,
    #[serde(rename = "LeftTop")]
    LeftTop,
    #[serde(rename = "RightTop")]
    RightTop,
    #[serde(rename = "RightBottom")]
    RightBottom,
    #[serde(rename = "LeftBottom")]
    LeftBottom,
}

