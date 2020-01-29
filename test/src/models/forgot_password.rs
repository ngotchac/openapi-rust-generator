use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForgotPassword {
    #[serde(rename(deserialize = "EnteredUsername"))]
    pub entered_username: String,
}


