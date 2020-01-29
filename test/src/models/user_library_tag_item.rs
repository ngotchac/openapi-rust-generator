use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserLibraryTagItem {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
}


