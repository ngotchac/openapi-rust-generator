use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbyReportsApiModelReportRow {
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
    #[serde(rename(deserialize = "HasImageTagsBackdrop"))]
    pub has_image_tags_backdrop: bool,
    #[serde(rename(deserialize = "HasImageTagsPrimary"))]
    pub has_image_tags_primary: bool,
    #[serde(rename(deserialize = "HasImageTagsLogo"))]
    pub has_image_tags_logo: bool,
    #[serde(rename(deserialize = "HasLocalTrailer"))]
    pub has_local_trailer: bool,
    #[serde(rename(deserialize = "HasLockData"))]
    pub has_lock_data: bool,
    #[serde(rename(deserialize = "HasEmbeddedImage"))]
    pub has_embedded_image: bool,
    #[serde(rename(deserialize = "HasSubtitles"))]
    pub has_subtitles: bool,
    #[serde(rename(deserialize = "HasSpecials"))]
    pub has_specials: bool,
    #[serde(rename(deserialize = "Columns"))]
    pub columns: Vec<Box<crate::models::EmbyReportsApiModelReportItem>>,
    #[serde(rename(deserialize = "RowType"))]
    pub row_type: RowType,
    #[serde(rename(deserialize = "UserId"))]
    pub user_id: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RowType {
    #[serde(rename = "MusicArtist")]
    MusicArtist,
    #[serde(rename = "MusicAlbum")]
    MusicAlbum,
    #[serde(rename = "Book")]
    Book,
    #[serde(rename = "BoxSet")]
    BoxSet,
    #[serde(rename = "Episode")]
    Episode,
    #[serde(rename = "Game")]
    Game,
    #[serde(rename = "Video")]
    Video,
    #[serde(rename = "Movie")]
    Movie,
    #[serde(rename = "MusicVideo")]
    MusicVideo,
    #[serde(rename = "Trailer")]
    Trailer,
    #[serde(rename = "Season")]
    Season,
    #[serde(rename = "Series")]
    Series,
    #[serde(rename = "Audio")]
    Audio,
    #[serde(rename = "BaseItem")]
    BaseItem,
    #[serde(rename = "Artist")]
    Artist,
}

