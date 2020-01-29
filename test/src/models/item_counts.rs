use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemCounts {
    #[serde(rename(deserialize = "MovieCount"))]
    pub movie_count: i32,
    #[serde(rename(deserialize = "SeriesCount"))]
    pub series_count: i32,
    #[serde(rename(deserialize = "EpisodeCount"))]
    pub episode_count: i32,
    #[serde(rename(deserialize = "GameCount"))]
    pub game_count: i32,
    #[serde(rename(deserialize = "ArtistCount"))]
    pub artist_count: i32,
    #[serde(rename(deserialize = "ProgramCount"))]
    pub program_count: i32,
    #[serde(rename(deserialize = "GameSystemCount"))]
    pub game_system_count: i32,
    #[serde(rename(deserialize = "TrailerCount"))]
    pub trailer_count: i32,
    #[serde(rename(deserialize = "SongCount"))]
    pub song_count: i32,
    #[serde(rename(deserialize = "AlbumCount"))]
    pub album_count: i32,
    #[serde(rename(deserialize = "MusicVideoCount"))]
    pub music_video_count: i32,
    #[serde(rename(deserialize = "BoxSetCount"))]
    pub box_set_count: i32,
    #[serde(rename(deserialize = "BookCount"))]
    pub book_count: i32,
    #[serde(rename(deserialize = "ItemCount"))]
    pub item_count: i32,
}


