use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogFile {
    #[serde(rename(deserialize = "DateCreated"))]
    pub date_created: String,
    #[serde(rename(deserialize = "DateModified"))]
    pub date_modified: String,
    #[serde(rename(deserialize = "Size"))]
    pub size: i64,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
}


