use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TasksTaskTriggerInfo {
    #[serde(rename(deserialize = "Type"))]
    pub _type: String,
    #[serde(rename(deserialize = "TimeOfDayTicks"), skip_serializing_if = "Option::is_none")]
    pub time_of_day_ticks: Option<i64>,
    #[serde(rename(deserialize = "IntervalTicks"), skip_serializing_if = "Option::is_none")]
    pub interval_ticks: Option<i64>,
    #[serde(rename(deserialize = "SystemEvent"))]
    pub system_event: SystemEvent,
    #[serde(rename(deserialize = "DayOfWeek"))]
    pub day_of_week: DayOfWeek,
    #[serde(rename(deserialize = "MaxRuntimeTicks"), skip_serializing_if = "Option::is_none")]
    pub max_runtime_ticks: Option<i64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SystemEvent {
    #[serde(rename = "WakeFromSleep")]
    WakeFromSleep,
    #[serde(rename = "DisplayConfigurationChange")]
    DisplayConfigurationChange,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DayOfWeek {
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

