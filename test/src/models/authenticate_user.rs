use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticateUser {
    #[serde(rename(deserialize = "Pw"))]
    pub pw: String,
    #[serde(rename(deserialize = "Password"))]
    pub password: String,
}


