use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbyReportsApiModelReportItem {
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Image"))]
    pub image: String,
    #[serde(rename(deserialize = "CustomTag"))]
    pub custom_tag: String,
}


