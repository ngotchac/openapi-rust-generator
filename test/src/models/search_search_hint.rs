use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchSearchHint {
    #[serde(rename(deserialize = "ItemId"))]
    pub item_id: i64,
    #[serde(rename(deserialize = "Id"))]
    pub id: i64,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "MatchedTerm"))]
    pub matched_term: String,
    #[serde(rename(deserialize = "IndexNumber"), skip_serializing_if = "Option::is_none")]
    pub index_number: Option<i32>,
    #[serde(rename(deserialize = "ProductionYear"), skip_serializing_if = "Option::is_none")]
    pub production_year: Option<i32>,
    #[serde(rename(deserialize = "ParentIndexNumber"), skip_serializing_if = "Option::is_none")]
    pub parent_index_number: Option<i32>,
    #[serde(rename(deserialize = "PrimaryImageTag"))]
    pub primary_image_tag: String,
    #[serde(rename(deserialize = "ThumbImageTag"))]
    pub thumb_image_tag: String,
    #[serde(rename(deserialize = "ThumbImageItemId"))]
    pub thumb_image_item_id: String,
    #[serde(rename(deserialize = "BackdropImageTag"))]
    pub backdrop_image_tag: String,
    #[serde(rename(deserialize = "BackdropImageItemId"))]
    pub backdrop_image_item_id: String,
    #[serde(rename(deserialize = "Type"))]
    pub _type: String,
    #[serde(rename(deserialize = "IsFolder"), skip_serializing_if = "Option::is_none")]
    pub is_folder: Option<bool>,
    #[serde(rename(deserialize = "RunTimeTicks"), skip_serializing_if = "Option::is_none")]
    pub run_time_ticks: Option<i64>,
    #[serde(rename(deserialize = "MediaType"))]
    pub media_type: String,
    #[serde(rename(deserialize = "StartDate"), skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename(deserialize = "EndDate"), skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename(deserialize = "Series"))]
    pub series: String,
    #[serde(rename(deserialize = "Status"))]
    pub status: String,
    #[serde(rename(deserialize = "Album"))]
    pub album: String,
    #[serde(rename(deserialize = "AlbumId"))]
    pub album_id: i64,
    #[serde(rename(deserialize = "AlbumArtist"))]
    pub album_artist: String,
    #[serde(rename(deserialize = "Artists"))]
    pub artists: Vec<String>,
    #[serde(rename(deserialize = "SongCount"), skip_serializing_if = "Option::is_none")]
    pub song_count: Option<i32>,
    #[serde(rename(deserialize = "EpisodeCount"), skip_serializing_if = "Option::is_none")]
    pub episode_count: Option<i32>,
    #[serde(rename(deserialize = "ChannelName"))]
    pub channel_name: String,
    #[serde(rename(deserialize = "PrimaryImageAspectRatio"), skip_serializing_if = "Option::is_none")]
    pub primary_image_aspect_ratio: Option<f64>,
}


