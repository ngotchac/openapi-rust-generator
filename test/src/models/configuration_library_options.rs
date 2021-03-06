use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationLibraryOptions {
    #[serde(rename(deserialize = "EnableArchiveMediaFiles"))]
    pub enable_archive_media_files: bool,
    #[serde(rename(deserialize = "EnablePhotos"))]
    pub enable_photos: bool,
    #[serde(rename(deserialize = "EnableRealtimeMonitor"))]
    pub enable_realtime_monitor: bool,
    #[serde(rename(deserialize = "EnableChapterImageExtraction"))]
    pub enable_chapter_image_extraction: bool,
    #[serde(rename(deserialize = "ExtractChapterImagesDuringLibraryScan"))]
    pub extract_chapter_images_during_library_scan: bool,
    #[serde(rename(deserialize = "DownloadImagesInAdvance"))]
    pub download_images_in_advance: bool,
    #[serde(rename(deserialize = "PathInfos"))]
    pub path_infos: Vec<Box<crate::models::ConfigurationMediaPathInfo>>,
    #[serde(rename(deserialize = "SaveLocalMetadata"))]
    pub save_local_metadata: bool,
    #[serde(rename(deserialize = "SaveLocalThumbnailSets"))]
    pub save_local_thumbnail_sets: bool,
    #[serde(rename(deserialize = "ImportMissingEpisodes"))]
    pub import_missing_episodes: bool,
    #[serde(rename(deserialize = "EnableAutomaticSeriesGrouping"))]
    pub enable_automatic_series_grouping: bool,
    #[serde(rename(deserialize = "EnableEmbeddedTitles"))]
    pub enable_embedded_titles: bool,
    #[serde(rename(deserialize = "EnableAudioResume"))]
    pub enable_audio_resume: bool,
    #[serde(rename(deserialize = "AutomaticRefreshIntervalDays"))]
    pub automatic_refresh_interval_days: i32,
    #[serde(rename(deserialize = "PreferredMetadataLanguage"))]
    pub preferred_metadata_language: String,
    #[serde(rename(deserialize = "ContentType"))]
    pub content_type: String,
    #[serde(rename(deserialize = "MetadataCountryCode"))]
    pub metadata_country_code: String,
    #[serde(rename(deserialize = "SeasonZeroDisplayName"))]
    pub season_zero_display_name: String,
    #[serde(rename(deserialize = "MetadataSavers"))]
    pub metadata_savers: Vec<String>,
    #[serde(rename(deserialize = "DisabledLocalMetadataReaders"))]
    pub disabled_local_metadata_readers: Vec<String>,
    #[serde(rename(deserialize = "LocalMetadataReaderOrder"))]
    pub local_metadata_reader_order: Vec<String>,
    #[serde(rename(deserialize = "DisabledSubtitleFetchers"))]
    pub disabled_subtitle_fetchers: Vec<String>,
    #[serde(rename(deserialize = "SubtitleFetcherOrder"))]
    pub subtitle_fetcher_order: Vec<String>,
    #[serde(rename(deserialize = "SkipSubtitlesIfEmbeddedSubtitlesPresent"))]
    pub skip_subtitles_if_embedded_subtitles_present: bool,
    #[serde(rename(deserialize = "SkipSubtitlesIfAudioTrackMatches"))]
    pub skip_subtitles_if_audio_track_matches: bool,
    #[serde(rename(deserialize = "SubtitleDownloadLanguages"))]
    pub subtitle_download_languages: Vec<String>,
    #[serde(rename(deserialize = "RequirePerfectSubtitleMatch"))]
    pub require_perfect_subtitle_match: bool,
    #[serde(rename(deserialize = "SaveSubtitlesWithMedia"))]
    pub save_subtitles_with_media: bool,
    #[serde(rename(deserialize = "ForcedSubtitlesOnly"))]
    pub forced_subtitles_only: bool,
    #[serde(rename(deserialize = "TypeOptions"))]
    pub type_options: Vec<Box<crate::models::ConfigurationTypeOptions>>,
    #[serde(rename(deserialize = "CollapseSingleItemFolders"))]
    pub collapse_single_item_folders: bool,
    #[serde(rename(deserialize = "MinResumePct"))]
    pub min_resume_pct: i32,
    #[serde(rename(deserialize = "MaxResumePct"))]
    pub max_resume_pct: i32,
    #[serde(rename(deserialize = "MinResumeDurationSeconds"))]
    pub min_resume_duration_seconds: i32,
    #[serde(rename(deserialize = "ThumbnailImagesIntervalSeconds"))]
    pub thumbnail_images_interval_seconds: i32,
}


