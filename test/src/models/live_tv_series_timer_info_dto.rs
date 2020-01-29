use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LiveTvSeriesTimerInfoDto {
    #[serde(rename(deserialize = "RecordAnyTime"))]
    pub record_any_time: bool,
    #[serde(rename(deserialize = "SkipEpisodesInLibrary"))]
    pub skip_episodes_in_library: bool,
    #[serde(rename(deserialize = "RecordAnyChannel"))]
    pub record_any_channel: bool,
    #[serde(rename(deserialize = "KeepUpTo"))]
    pub keep_up_to: i32,
    #[serde(rename(deserialize = "RecordNewOnly"))]
    pub record_new_only: bool,
    #[serde(rename(deserialize = "Days"))]
    pub days: Days,
    #[serde(rename(deserialize = "DayPattern"))]
    pub day_pattern: DayPattern,
    #[serde(rename(deserialize = "ImageTags"))]
    pub image_tags: ::std::collections::HashMap<String, String>,
    #[serde(rename(deserialize = "ParentThumbItemId"))]
    pub parent_thumb_item_id: String,
    #[serde(rename(deserialize = "ParentThumbImageTag"))]
    pub parent_thumb_image_tag: String,
    #[serde(rename(deserialize = "ParentPrimaryImageItemId"))]
    pub parent_primary_image_item_id: String,
    #[serde(rename(deserialize = "ParentPrimaryImageTag"))]
    pub parent_primary_image_tag: String,
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
pub enum Days {
    #[serde(rename = "Sunday")]
    Sunday,
    #[serde(rename = "Monday")]
    Monday,
    #[serde(rename = "Tuesday")]
    Tuesday,
    #[serde(rename = "Wednesday")]
    Wednesday,
    #[serde(rename = "Thursday")]
    Thursday,
    #[serde(rename = "Friday")]
    Friday,
    #[serde(rename = "Saturday")]
    Saturday,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DayPattern {
    #[serde(rename = "Daily")]
    Daily,
    #[serde(rename = "Weekdays")]
    Weekdays,
    #[serde(rename = "Weekends")]
    Weekends,
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

