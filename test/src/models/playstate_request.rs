use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlaystateRequest {
    #[serde(rename(deserialize = "Command"))]
    pub command: Command,
    #[serde(rename(deserialize = "SeekPositionTicks"), skip_serializing_if = "Option::is_none")]
    pub seek_position_ticks: Option<i64>,
    #[serde(rename(deserialize = "ControllingUserId"))]
    pub controlling_user_id: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Command {
    #[serde(rename = "Stop")]
    Stop,
    #[serde(rename = "Pause")]
    Pause,
    #[serde(rename = "Unpause")]
    Unpause,
    #[serde(rename = "NextTrack")]
    NextTrack,
    #[serde(rename = "PreviousTrack")]
    PreviousTrack,
    #[serde(rename = "Seek")]
    Seek,
    #[serde(rename = "Rewind")]
    Rewind,
    #[serde(rename = "FastForward")]
    FastForward,
    #[serde(rename = "PlayPause")]
    PlayPause,
}

