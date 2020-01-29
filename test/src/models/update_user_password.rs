use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateUserPassword {
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
    #[serde(rename(deserialize = "CurrentPw"))]
    pub current_pw: String,
    #[serde(rename(deserialize = "NewPw"))]
    pub new_pw: String,
    #[serde(rename(deserialize = "ResetPassword"))]
    pub reset_password: bool,
}


