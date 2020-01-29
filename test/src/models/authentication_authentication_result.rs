use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticationAuthenticationResult {
    #[serde(rename(deserialize = "User"))]
    pub user: Box<crate::models::UserDto>,
    #[serde(rename(deserialize = "SessionInfo"))]
    pub session_info: Box<crate::models::SessionSessionInfo>,
    #[serde(rename(deserialize = "AccessToken"))]
    pub access_token: String,
    #[serde(rename(deserialize = "ServerId"))]
    pub server_id: String,
}


