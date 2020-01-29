use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectUserLinkResult {
    #[serde(rename(deserialize = "IsPending"))]
    pub is_pending: bool,
    #[serde(rename(deserialize = "IsNewUserInvitation"))]
    pub is_new_user_invitation: bool,
    #[serde(rename(deserialize = "GuestDisplayName"))]
    pub guest_display_name: String,
}


