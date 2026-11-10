use std::fs;
use std::io::Error;
use std::path::PathBuf;
use chrono::{DateTime, Local};
use confy::ConfyError;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct TOMLEmpty {
    pub error: String,
}

// Toml Data V1
#[derive(Serialize,Deserialize, Debug)]
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

#[derive(Serialize,Deserialize, Debug)]
struct DailyV1 {
    title: String,
    description: String,
    last_completed: DateTime<Local>,
}
#[derive(Serialize,Deserialize, Debug)]
struct ToNotV1 {
    title: String,
    description: String,
    last_done: DateTime<Local>,
}
#[derive(Serialize,Deserialize, Debug, Clone)]
struct TasksV1 {
    title: String,
    description: String,
    children: Vec<TasksV1>,
    completed: bool,
    category : bool
}


// Universal
#[derive(Serialize ,Deserialize, Debug)]
#[serde(tag = "version", rename_all = "kebab-case")]
pub(crate) enum HomeData {
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

    pub fn v1() -> DataHandler {
        DataHandler {
            app_data: HomeData::V1(TOMLdataV1::default()),
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