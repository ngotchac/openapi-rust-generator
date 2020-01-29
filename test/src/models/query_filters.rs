use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryFilters {
    #[serde(rename(deserialize = "Genres"))]
    pub genres: Vec<Box<crate::models::NameLongIdPair>>,
    #[serde(rename(deserialize = "Studios"))]
    pub studios: Vec<Box<crate::models::NameLongIdPair>>,
    #[serde(rename(deserialize = "Tags"))]
    pub tags: Vec<String>,
}


