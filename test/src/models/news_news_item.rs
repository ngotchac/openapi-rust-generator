use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NewsNewsItem {
    #[serde(rename(deserialize = "Title"))]
    pub title: String,
    #[serde(rename(deserialize = "Link"))]
    pub link: String,
    #[serde(rename(deserialize = "Description"))]
    pub description: String,
    #[serde(rename(deserialize = "DescriptionHtml"))]
    pub description_html: String,
    #[serde(rename(deserialize = "Guid"))]
    pub guid: String,
    #[serde(rename(deserialize = "Date"))]
    pub date: String,
}


