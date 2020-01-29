use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsersPinRedeemResult {
    #[serde(rename(deserialize = "Success"))]
    pub success: bool,
    #[serde(rename(deserialize = "UsersReset"))]
    pub users_reset: Vec<String>,
}


