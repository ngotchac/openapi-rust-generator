use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeneralCommand {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "ControllingUserId"))]
    pub controlling_user_id: String,
    #[serde(rename(deserialize = "Arguments"))]
    pub arguments: ::std::collections::HashMap<String, String>,
}


