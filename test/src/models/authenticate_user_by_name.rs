use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticateUserByName {
    #[serde(rename(deserialize = "Username"))]
    pub username: String,
    #[serde(rename(deserialize = "Password"))]
    pub password: String,
    #[serde(rename(deserialize = "Pw"))]
    pub pw: String,
}


