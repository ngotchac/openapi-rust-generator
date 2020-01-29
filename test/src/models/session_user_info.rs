use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionUserInfo {
    #[serde(rename(deserialize = "UserId"))]
    pub user_id: String,
    #[serde(rename(deserialize = "UserName"))]
    pub user_name: String,
    #[serde(rename(deserialize = "UserInternalId"))]
    pub user_internal_id: i64,
}


