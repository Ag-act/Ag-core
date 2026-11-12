pub mod v1;

use std::fs;
use v1::{HomeDataV1, TimeSinceV1, TasksV1};
use std::path::PathBuf;
use chrono::{DateTime, Local};
use confy::ConfyError;
use serde::{Deserialize, Serialize};
use crate::v1::DailyV1;

#[derive(Serialize, Deserialize)]
pub struct Config {
    version: AgactVersions,
    home: Vec<PathBuf>,
    projects: Vec<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version : AgactVersions::V1,
            home : Vec::new(),
            projects : Vec::new()
        }
    }
}

impl Config {
    pub fn write(&self) -> Result<(), ConfyError> {
        confy::store("Ag-act", None, self)?;
        Ok(())
    }

    pub fn load() -> Result<Self, ConfyError> {
        confy::load("Ag-act", None)
    }

    pub fn set_version(&mut self, versions: AgactVersions) {
        self.version = versions;
    }

    pub fn set_homes(&mut self, home: Vec<PathBuf>) {
        self.home = home;
    }

    pub fn set_projects(&mut self, projects: Vec<PathBuf>) {
        self.projects = projects;
    }

    pub fn get_projects(&self) -> Vec<PathBuf> {
        self.projects.clone()
    }

    pub fn get_version(&self) -> &AgactVersions {
        &self.version
    }

    pub fn get_homes(&self) -> Vec<PathBuf> {
        self.home.clone()
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Default)]
pub enum AgactVersions {
    #[default]
    V1
}

// Universal
#[derive(Serialize ,Deserialize, Debug)]
#[serde(tag = "version", rename_all = "kebab-case")]
pub(crate) enum HomeData {
    #[serde(rename = "1")]
    V1(HomeDataV1),
}

#[derive(Clone)]
pub struct TasksUniversal {
     title: String,
     description: String,
     identifier: String,
     completed: bool,
     children: Vec<TasksUniversal>,
     category: bool,
}
impl TasksUniversal {
    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn get_identifier(&self) -> String {
        self.identifier.clone()
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn get_children(&self) -> Vec<TasksUniversal> {
        self.children.clone()
    }

    pub fn is_category(&self) -> bool {
        self.category
    }
}
pub struct DailyUniversal {
     title: String,
     description: String,
     identifier: String,
     last_completed: DateTime<Local>,
}
impl DailyUniversal {
    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn get_identifier(&self) -> String {
        self.identifier.clone()
    }

    pub fn last_completed(&self) -> DateTime<Local> {
        self.last_completed.clone()
    }
}
pub struct ToNotUniversal {
     title: String,
     description: String,
     identifier: String,
     last_completed: DateTime<Local>,
}
impl ToNotUniversal {
    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn get_identifier(&self) -> String {
        self.identifier.clone()
    }

    pub fn last_completed(&self) -> DateTime<Local> {
        self.last_completed.clone()
    }
}

impl From<&TimeSinceV1> for ToNotUniversal {
    fn from(v: &TimeSinceV1) -> Self {
        ToNotUniversal {
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

pub struct DataHandler {
    app_data: HomeData,
}

impl DataHandler {
    pub fn new(data : &String) -> Result<DataHandler, toml::de::Error> {
        let data = DataHandler {
            app_data: {
                toml::from_str(&data)?
            },
        };
        Ok(data)
    }
    pub fn default(config: Config) -> DataHandler {
        match config.version {
            AgactVersions::V1 => {
                Self::v1()
            }
        }
    }
    pub fn v1() -> DataHandler {
        DataHandler {
            app_data: HomeData::V1(HomeDataV1::default()),
        }
    }

    pub fn get_daily_tasks(&self) -> Vec<DailyUniversal> {
        match &self.app_data {
            HomeData::V1(data) => {
                data.daily_tasks.iter().map(DailyUniversal::from).collect()
            }
        }
    }

    pub fn get_to_nots(&self) -> Vec<ToNotUniversal> {
        match &self.app_data {
            HomeData::V1(data) => {
                data.to_not.iter().map(ToNotUniversal::from).collect()
            }
        }
    }

    pub fn get_tasks(&self) -> Vec<TasksUniversal> {
        match &self.app_data {
            HomeData::V1(data) => {
                data.tasks.iter().map(TasksUniversal::from).collect()
            }
        }
    }

    pub fn write(&self, path_buf: PathBuf) -> Result<(), toml::ser::Error>{
        fs::write(path_buf.as_path(), toml::to_string(&self.app_data)?).expect("ERROR: Write Failed");
        Ok(())
    }
}