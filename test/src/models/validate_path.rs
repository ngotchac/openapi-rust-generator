use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidatePath {
    #[serde(rename(deserialize = "ValidateWriteable"))]
    pub validate_writeable: bool,
    #[serde(rename(deserialize = "IsFile"), skip_serializing_if = "Option::is_none")]
    pub is_file: Option<bool>,
}


