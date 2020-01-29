use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForgotPasswordPin {
    #[serde(rename(deserialize = "Pin"))]
    pub pin: String,
}


