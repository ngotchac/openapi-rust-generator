use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationAccessSchedule {
    #[serde(rename(deserialize = "DayOfWeek"))]
    pub day_of_week: DayOfWeek,
    #[serde(rename(deserialize = "StartHour"))]
    pub start_hour: f64,
    #[serde(rename(deserialize = "EndHour"))]
    pub end_hour: f64,
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
    #[serde(rename = "Everyday")]
    Everyday,
    #[serde(rename = "Weekday")]
    Weekday,
    #[serde(rename = "Weekend")]
    Weekend,
}

