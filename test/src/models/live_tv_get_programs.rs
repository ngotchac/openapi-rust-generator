use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LiveTvGetPrograms {
    #[serde(rename(deserialize = "IsAiring"), skip_serializing_if = "Option::is_none")]
    pub is_airing: Option<bool>,
    #[serde(rename(deserialize = "EnableTotalRecordCount"))]
    pub enable_total_record_count: bool,
    #[serde(rename(deserialize = "SeriesTimerId"))]
    pub series_timer_id: String,
    #[serde(rename(deserialize = "LibrarySeriesId"))]
    pub library_series_id: String,
}


