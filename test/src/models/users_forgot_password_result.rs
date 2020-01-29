use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsersForgotPasswordResult {
    #[serde(rename(deserialize = "Action"))]
    pub action: Action,
    #[serde(rename(deserialize = "PinFile"))]
    pub pin_file: String,
    #[serde(rename(deserialize = "PinExpirationDate"), skip_serializing_if = "Option::is_none")]
    pub pin_expiration_date: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "ContactAdmin")]
    ContactAdmin,
    #[serde(rename = "PinCode")]
    PinCode,
    #[serde(rename = "InNetworkRequired")]
    InNetworkRequired,
}

