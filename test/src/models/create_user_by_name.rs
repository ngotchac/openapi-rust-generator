use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateUserByName {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
}


