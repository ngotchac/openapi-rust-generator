use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationUserConfiguration {
    #[serde(rename(deserialize = "AudioLanguagePreference"))]
    pub audio_language_preference: String,
    #[serde(rename(deserialize = "PlayDefaultAudioTrack"))]
    pub play_default_audio_track: bool,
    #[serde(rename(deserialize = "SubtitleLanguagePreference"))]
    pub subtitle_language_preference: String,
    #[serde(rename(deserialize = "DisplayMissingEpisodes"))]
    pub display_missing_episodes: bool,
    #[serde(rename(deserialize = "GroupedFolders"))]
    pub grouped_folders: Vec<String>,
    #[serde(rename(deserialize = "SubtitleMode"))]
    pub subtitle_mode: SubtitleMode,
    #[serde(rename(deserialize = "DisplayCollectionsView"))]
    pub display_collections_view: bool,
    #[serde(rename(deserialize = "EnableLocalPassword"))]
    pub enable_local_password: bool,
    #[serde(rename(deserialize = "OrderedViews"))]
    pub ordered_views: Vec<String>,
    #[serde(rename(deserialize = "LatestItemsExcludes"))]
    pub latest_items_excludes: Vec<String>,
    #[serde(rename(deserialize = "MyMediaExcludes"))]
    pub my_media_excludes: Vec<String>,
    #[serde(rename(deserialize = "HidePlayedInLatest"))]
    pub hide_played_in_latest: bool,
    #[serde(rename(deserialize = "RememberAudioSelections"))]
    pub remember_audio_selections: bool,
    #[serde(rename(deserialize = "RememberSubtitleSelections"))]
    pub remember_subtitle_selections: bool,
    #[serde(rename(deserialize = "EnableNextEpisodeAutoPlay"))]
    pub enable_next_episode_auto_play: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SubtitleMode {
    #[serde(rename = "Default")]
    Default,
    #[serde(rename = "Always")]
    Always,
    #[serde(rename = "OnlyForced")]
    OnlyForced,
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Smart")]
    Smart,
}

