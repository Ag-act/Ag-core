use std::io::Error;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Default)]
pub enum AgactVersions {
    #[default]
    V1
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct TOMLEmpty {
    pub error: String,
}

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
    category : bool
}


// Universal
#[derive(Deserialize, Debug)]
#[serde(tag = "version", rename_all = "kebab-case")]
pub(crate) enum AppData {
    #[serde(rename = "0")]
    Empty(TOMLEmpty),
    #[serde(rename = "1")]
    V1(TOMLdataV1),
}

pub struct TasksUniversal {
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub children: Vec<TasksUniversal>,
    pub category: bool,
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
            category: value.category.clone()
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
                toml::from_str(&data).unwrap_or(AppData::Empty(TOMLEmpty{error : "Failed To Parse Data".into()}))
            }
        }
    }

    pub fn v1(&self) -> DataHandler {
        DataHandler {
            app_data: AppData::V1(TOMLdataV1::default())
        }
    }

    pub fn empty() -> DataHandler {
        DataHandler {
            app_data: AppData::Empty(TOMLEmpty::default())
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.app_data {
            AppData::Empty(_) => true,
            _ => false
        }
    }

    pub fn get_empty_message(&self) -> String {
        match &self.app_data {
            AppData::Empty(d) => d.error.clone(),
            _ => "".to_string(),
        }
    }

    pub fn get_daily_tasks(&self) -> Vec<DailyUniversal> {
        match &self.app_data {
            AppData::V1(data) => {
                data.daily_tasks.iter().map(DailyUniversal::from).collect()
            }
            _ => {Vec::new()}
        }
    }

    pub fn get_to_nots(&self) -> Vec<ToNotUniversal> {
        match &self.app_data {
            AppData::V1(data) => {
                data.to_not.iter().map(ToNotUniversal::from).collect()
            }
            _ => {Vec::new()}
        }
    }

    pub fn get_tasks(&self) -> Vec<TasksUniversal> {
        match &self.app_data {
            AppData::V1(data) => {
                data.tasks.iter().map(TasksUniversal::from).collect()
            }
            _ => {Vec::new()}
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Settings {
    pub file_path: String,
    pub version: AgactVersions,
}