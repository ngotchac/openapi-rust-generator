use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectConnectAuthenticationExchangeResult {
    #[serde(rename(deserialize = "LocalUserId"))]
    pub local_user_id: String,
    #[serde(rename(deserialize = "AccessToken"))]
    pub access_token: String,
}


