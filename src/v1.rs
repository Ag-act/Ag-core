use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize, Debug)]
pub(crate) struct HomeDataV1 {
    pub daily_tasks: Vec<DailyV1>,
    pub to_not: Vec<TimeSinceV1>,
    pub tasks: Vec<TasksV1>,
}

impl Default for HomeDataV1 {
    fn default() -> Self {
        Self {daily_tasks: vec![], to_not: vec![], tasks: vec![]}
    }
}

#[derive(Serialize,Deserialize, Debug)]
pub(crate) struct DailyV1 {
    pub title: String,
    pub description: String,
    pub last_completed: DateTime<Local>,
}
#[derive(Serialize,Deserialize, Debug)]
pub(crate) struct TimeSinceV1 {
    pub title: String,
    pub description: String,
    pub last_done: DateTime<Local>,
}
#[derive(Serialize,Deserialize, Debug, Clone)]
pub(crate) struct TasksV1 {
    pub title: String,
    pub description: String,
    pub children: Vec<TasksV1>,
    pub completed: bool,
    pub category : bool
}