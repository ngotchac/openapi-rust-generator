use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LiveTvTimerInfoDto {
    #[serde(rename(deserialize = "Status"))]
    pub status: Status,
    #[serde(rename(deserialize = "SeriesTimerId"))]
    pub series_timer_id: String,
    #[serde(rename(deserialize = "ExternalSeriesTimerId"))]
    pub external_series_timer_id: String,
    #[serde(rename(deserialize = "RunTimeTicks"), skip_serializing_if = "Option::is_none")]
    pub run_time_ticks: Option<i64>,
    #[serde(rename(deserialize = "ProgramInfo"))]
    pub program_info: Box<crate::models::BaseItemDto>,
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
    #[serde(rename(deserialize = "Type"))]
    pub _type: String,
    #[serde(rename(deserialize = "ServerId"))]
    pub server_id: String,
    #[serde(rename(deserialize = "ExternalId"))]
    pub external_id: String,
    #[serde(rename(deserialize = "ChannelId"))]
    pub channel_id: String,
    #[serde(rename(deserialize = "ExternalChannelId"))]
    pub external_channel_id: String,
    #[serde(rename(deserialize = "ChannelName"))]
    pub channel_name: String,
    #[serde(rename(deserialize = "ChannelPrimaryImageTag"))]
    pub channel_primary_image_tag: String,
    #[serde(rename(deserialize = "ProgramId"))]
    pub program_id: String,
    #[serde(rename(deserialize = "ExternalProgramId"))]
    pub external_program_id: String,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Overview"))]
    pub overview: String,
    #[serde(rename(deserialize = "StartDate"))]
    pub start_date: String,
    #[serde(rename(deserialize = "EndDate"))]
    pub end_date: String,
    #[serde(rename(deserialize = "ServiceName"))]
    pub service_name: String,
    #[serde(rename(deserialize = "Priority"))]
    pub priority: i32,
    #[serde(rename(deserialize = "PrePaddingSeconds"))]
    pub pre_padding_seconds: i32,
    #[serde(rename(deserialize = "PostPaddingSeconds"))]
    pub post_padding_seconds: i32,
    #[serde(rename(deserialize = "IsPrePaddingRequired"))]
    pub is_pre_padding_required: bool,
    #[serde(rename(deserialize = "ParentBackdropItemId"))]
    pub parent_backdrop_item_id: String,
    #[serde(rename(deserialize = "ParentBackdropImageTags"))]
    pub parent_backdrop_image_tags: Vec<String>,
    #[serde(rename(deserialize = "IsPostPaddingRequired"))]
    pub is_post_padding_required: bool,
    #[serde(rename(deserialize = "KeepUntil"))]
    pub keep_until: KeepUntil,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "New")]
    New,
    #[serde(rename = "InProgress")]
    InProgress,
    #[serde(rename = "Completed")]
    Completed,
    #[serde(rename = "Cancelled")]
    Cancelled,
    #[serde(rename = "ConflictedOk")]
    ConflictedOk,
    #[serde(rename = "ConflictedNotOk")]
    ConflictedNotOk,
    #[serde(rename = "Error")]
    Error,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum KeepUntil {
    #[serde(rename = "UntilDeleted")]
    UntilDeleted,
    #[serde(rename = "UntilSpaceNeeded")]
    UntilSpaceNeeded,
    #[serde(rename = "UntilWatched")]
    UntilWatched,
    #[serde(rename = "UntilDate")]
    UntilDate,
}

