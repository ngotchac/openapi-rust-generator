use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllThemeMediaResult {
    #[serde(rename(deserialize = "ThemeVideosResult"))]
    pub theme_videos_result: Box<crate::models::ThemeMediaResult>,
    #[serde(rename(deserialize = "ThemeSongsResult"))]
    pub theme_songs_result: Box<crate::models::ThemeMediaResult>,
    #[serde(rename(deserialize = "SoundtrackSongsResult"))]
    pub soundtrack_songs_result: Box<crate::models::ThemeMediaResult>,
}


