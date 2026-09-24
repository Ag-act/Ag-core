use chrono::{DateTime, Local};
use serde::Deserialize;

// Toml Data V1
#[derive(Deserialize, Debug)]
pub(crate) struct TOMLdataV1 {
    daily_tasks: Vec<DailyV1>,
    to_not: Vec<ToNotV1>,
    tasks: Vec<TasksV1>,
}

impl Default for TOMLdataV1 {
    fn default() -> Self {
        Self {daily_tasks: vec![], to_not: vec![], tasks: vec![]}
    }
}

#[derive(Deserialize, Debug)]
struct DailyV1 {
    title: String,
    description: String,
    last_completed: DateTime<Local>,
}
#[derive(Deserialize, Debug)]
struct ToNotV1 {
    title: String,
    description: String,
    last_done: DateTime<Local>,
}
#[derive(Deserialize, Debug, Clone)]
struct TasksV1 {
    title: String,
    description: String,
    children: Vec<TasksV1>,
    completed: bool,
}


// Universal
#[derive(Deserialize, Debug)]
#[serde(tag = "version", rename_all = "kebab-case")]
pub(crate) enum AppData {
    #[serde(rename = "1")]
    V1(TOMLdataV1)
}

pub struct TasksUniversal {
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub children: Vec<TasksUniversal>,
}
pub struct DailyUniversal {
    pub title: String,
    pub description: String,
    pub last_completed: DateTime<Local>,
}
pub struct ToNotUniversal {
    pub title: String,
    pub description: String,
    pub last_completed: DateTime<Local>,
}

impl From<&ToNotV1> for ToNotUniversal {
    fn from(v: &ToNotV1) -> Self {
        ToNotUniversal {
            title: v.title.clone(),
            description: v.description.clone(),
            last_completed: v.last_done.clone(),
        }
    }
}
impl From<&DailyV1> for DailyUniversal {
    fn from(value: &DailyV1) -> Self {
        DailyUniversal {
            title: value.title.clone(),
            description: value.description.clone(),
            last_completed: value.last_completed.clone(),
        }
    }
}
impl From<&TasksV1> for TasksUniversal {
    fn from(value: &TasksV1) -> Self {
        Self {
            title: value.title.clone(),
            description: value.description.clone(),
            completed: value.completed.clone(),
            children: value.children.iter().map(|v| v.into()).collect(),
        }
    }
}

pub struct DataHandler {
    app_data: AppData
}
impl Default for DataHandler {
    fn default() -> Self {
        Self {app_data: AppData::V1(TOMLdataV1::default())}
    }
}

impl DataHandler {
    pub fn new(data : &String) -> DataHandler {
        DataHandler {
            app_data: {
                toml::from_str(&data).unwrap()
            }
        }
    }

    pub fn v1(&self) -> DataHandler {
        DataHandler {
            app_data: AppData::V1(TOMLdataV1::default())
        }
    }

    fn get_daily_tasks(&self) -> Vec<DailyUniversal> {
        match &self.app_data {
            AppData::V1(data) => {
                data.daily_tasks.iter().map(DailyUniversal::from).collect()
            }
        }
    }

    fn get_to_nots(&self) -> Vec<ToNotUniversal> {
        match &self.app_data {
            AppData::V1(data) => {
                data.to_not.iter().map(ToNotUniversal::from).collect()
            }
        }
    }

    fn get_tasks(&self) -> Vec<TasksUniversal> {
        match &self.app_data {
            AppData::V1(data) => {
                data.tasks.iter().map(TasksUniversal::from).collect()
            }
        }
    }
}