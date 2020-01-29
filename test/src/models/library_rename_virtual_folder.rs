use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LibraryRenameVirtualFolder {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "NewName"))]
    pub new_name: String,
    #[serde(rename(deserialize = "RefreshLibrary"))]
    pub refresh_library: bool,
}


