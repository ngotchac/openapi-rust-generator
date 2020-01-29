use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GameSystemSummary {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "DisplayName"))]
    pub display_name: String,
    #[serde(rename(deserialize = "GameCount"))]
    pub game_count: i32,
    #[serde(rename(deserialize = "GameFileExtensions"))]
    pub game_file_extensions: Vec<String>,
    #[serde(rename(deserialize = "ClientInstalledGameCount"))]
    pub client_installed_game_count: i32,
}


