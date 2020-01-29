use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlaylistsPlaylistCreationResult {
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
}


