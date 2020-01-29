use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TasksTaskInfo {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "State"))]
    pub state: State,
    #[serde(rename(deserialize = "CurrentProgressPercentage"), skip_serializing_if = "Option::is_none")]
    pub current_progress_percentage: Option<f64>,
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
    #[serde(rename(deserialize = "LastExecutionResult"))]
    pub last_execution_result: Box<crate::models::TasksTaskResult>,
    #[serde(rename(deserialize = "Triggers"))]
    pub triggers: Vec<Box<crate::models::TasksTaskTriggerInfo>>,
    #[serde(rename(deserialize = "Description"))]
    pub description: String,
    #[serde(rename(deserialize = "Category"))]
    pub category: String,
    #[serde(rename(deserialize = "IsHidden"))]
    pub is_hidden: bool,
    #[serde(rename(deserialize = "Key"))]
    pub key: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "Idle")]
    Idle,
    #[serde(rename = "Cancelling")]
    Cancelling,
    #[serde(rename = "Running")]
    Running,
}

