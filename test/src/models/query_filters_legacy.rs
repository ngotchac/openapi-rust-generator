use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryFiltersLegacy {
    #[serde(rename(deserialize = "Genres"))]
    pub genres: Vec<String>,
    #[serde(rename(deserialize = "Tags"))]
    pub tags: Vec<String>,
    #[serde(rename(deserialize = "OfficialRatings"))]
    pub official_ratings: Vec<String>,
    #[serde(rename(deserialize = "Years"))]
    pub years: Vec<i32>,
}


