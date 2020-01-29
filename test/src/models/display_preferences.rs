use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DisplayPreferences {
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
    #[serde(rename(deserialize = "ViewType"))]
    pub view_type: String,
    #[serde(rename(deserialize = "SortBy"))]
    pub sort_by: String,
    #[serde(rename(deserialize = "IndexBy"))]
    pub index_by: String,
    #[serde(rename(deserialize = "RememberIndexing"))]
    pub remember_indexing: bool,
    #[serde(rename(deserialize = "PrimaryImageHeight"))]
    pub primary_image_height: i32,
    #[serde(rename(deserialize = "PrimaryImageWidth"))]
    pub primary_image_width: i32,
    #[serde(rename(deserialize = "CustomPrefs"))]
    pub custom_prefs: ::std::collections::HashMap<String, String>,
    #[serde(rename(deserialize = "ScrollDirection"))]
    pub scroll_direction: ScrollDirection,
    #[serde(rename(deserialize = "ShowBackdrop"))]
    pub show_backdrop: bool,
    #[serde(rename(deserialize = "RememberSorting"))]
    pub remember_sorting: bool,
    #[serde(rename(deserialize = "SortOrder"))]
    pub sort_order: SortOrder,
    #[serde(rename(deserialize = "ShowSidebar"))]
    pub show_sidebar: bool,
    #[serde(rename(deserialize = "Client"))]
    pub client: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ScrollDirection {
    #[serde(rename = "Horizontal")]
    Horizontal,
    #[serde(rename = "Vertical")]
    Vertical,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SortOrder {
    #[serde(rename = "Ascending")]
    Ascending,
    #[serde(rename = "Descending")]
    Descending,
}

