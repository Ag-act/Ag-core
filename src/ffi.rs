use std::fs;
use std::sync::RwLock;
use std::time::SystemTime;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use crate::{AgactVersions, HomeData};
use crate::errors::AgError;
use crate::v1::{gen_id_v1, TimeSinceV1, DailyV1, TasksV1};

#[derive(Deserialize, Serialize, Default)]
#[serde(default)]
#[derive(uniffi::Record)]
pub struct Config {
    version : AgactVersions,
    home : String,
    projects : Vec<String>
}

#[uniffi::export]
pub fn load_config() -> Result<Config, AgError> {
    confy::load("Ag-act", None).map_err(|e| AgError::ConfigReadError(e.to_string()))
}

#[uniffi::export]
pub fn write_config(config: Config) -> Result<(), AgError> {
    confy::store("Ag-act", None, &config).map_err(|e| AgError::ConfigWriteError(e.to_string()))
}

#[derive(uniffi::Object)]
pub struct DataHandler {
    inner : RwLock<crate::DataHandler>,
}

#[uniffi::export]
pub fn load_data(data : &String) -> Result<DataHandler, AgError> {
    let data = DataHandler {
        inner: RwLock::new(crate::DataHandler::new(data).map_err(|e| AgError::HomeReadError(e.to_string()))?),
    };
    Ok(data)
}

#[uniffi::export]
impl DataHandler {
    #[uniffi::constructor]
    pub fn default(config: &Config) -> DataHandler {
        match config.version {
            AgactVersions::V1 => {
                Self::v1()
            }
        }
    }

    #[uniffi::constructor]
    pub fn v1() -> DataHandler {
        DataHandler {
            inner: RwLock::new(crate::DataHandler::v1()),
        }
    }

    pub fn get_daily_tasks(&self) -> Vec<DailyUniversal> {
        let data = self.inner.read().unwrap();
        match &data.app_data {
            HomeData::V1(data) => {
                data.daily_tasks.iter().map(DailyUniversal::from).collect()
            }
        }
    }

    pub fn get_to_nots(&self) -> Vec<TimeSinceUniversal> {
        let data = self.inner.read().unwrap();
        match &data.app_data {
            HomeData::V1(data) => {
                data.to_not.iter().map(TimeSinceUniversal::from).collect()
            }
        }
    }
    pub fn get_tasks(&self) -> Vec<TasksUniversal> {
        let data = self.inner.read().unwrap();
        match &data.app_data {
            HomeData::V1(data) => {
                data.tasks.iter().map(TasksUniversal::from).collect()
            }
        }
    }
    pub fn add_task(&self, task: TasksUniversal) -> String {
        let mut data = self.inner.write().unwrap();
        match &mut data.app_data {
            HomeData::V1(data) => {
                let id = gen_id_v1(&data);
                data.tasks.push(task.clone().into());
                id
            }
        }
    }

    pub fn add_to_not(&self, task: TimeSinceUniversal) -> String {
        let mut data = self.inner.write().unwrap();
        match &mut data.app_data {
            HomeData::V1(data) => {
                let id = gen_id_v1(&data);

                data.to_not.push(task.clone().into());
                id
            }
        }
    }

    pub fn  add_daily(&self, task: DailyUniversal) -> String {
        let mut data = self.inner.write().unwrap();
        match &mut data.app_data {
            HomeData::V1(data) => {
                let id = gen_id_v1(&data);

                data.daily_tasks.push(task.into());
                id
            }
        }
    }

    pub fn write(&self, string: String) -> Result<(), AgError> {
        let data = self.inner.read().unwrap();
        fs::write(string, toml::to_string(&data.app_data).map_err(|e| AgError::ConfigWriteError(e.to_string()))?);
        Ok(())
    }
}

#[derive(Clone, uniffi::Record)]
pub struct TasksUniversal {
    title: String,
    description: String,
    identifier: String,
    pub completed: bool,
    children: Vec<TasksUniversal>,
    category: bool,
}
#[derive(Clone, uniffi::Record)]
pub struct DailyUniversal {
    title: String,
    description: String,
    identifier: String,
    last_completed: SystemTime,
}

#[derive(Clone, uniffi::Record)]
pub struct TimeSinceUniversal {
    title: String,
    description: String,
    identifier: String,
    last_completed: SystemTime,
}

impl From<&TimeSinceV1> for TimeSinceUniversal {
    fn from(v: &TimeSinceV1) -> Self {
        TimeSinceUniversal {
            title: v.title.clone(),
            description: v.description.clone(),
            last_completed: v.last_done.clone(),
            identifier: v.identifier.clone(),
        }
    }
}
impl From<&DailyV1> for DailyUniversal {
    fn from(value: &DailyV1) -> Self {
        DailyUniversal {
            title: value.title.clone(),
            description: value.description.clone(),
            last_completed: value.last_completed.clone(),
            identifier: value.identifier.clone(),
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
            category: value.category.clone(),
            identifier: value.identifier.clone(),
        }
    }
}
impl From<TasksUniversal> for TasksV1 {
    fn from(v: TasksUniversal) -> Self {
        Self {
            title: v.title.clone(),
            description: v.description.clone(),
            children: v.children.iter().map(|v| v.clone().into()).collect(),
            category: v.category,
            completed: v.completed,
            identifier: v.identifier,
        }
    }
}
impl From<TimeSinceUniversal> for TimeSinceV1 {
    fn from(v: TimeSinceUniversal) -> Self {
        Self {
            title: v.title,
            description: v.description,
            identifier: v.identifier,
            last_done: v.last_completed,
        }
    }
}
impl From<DailyUniversal> for DailyV1 {
    fn from(v: DailyUniversal) -> Self {
        Self {
            title: v.title,
            description: v.description,
            last_completed: v.last_completed,
            identifier: v.identifier,
        }
    }
}
