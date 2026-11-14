pub mod v1;
pub mod universal;

use std::fs;
use v1::{HomeDataV1};
use std::path::PathBuf;
use universal::{DailyUniversal,TasksUniversal,TimeSinceUniversal};
use confy::ConfyError;
use serde::{Deserialize, Serialize};
use crate::v1::{gen_id_v1};

#[derive(Serialize, Deserialize)]
pub struct Config {
    version: AgactVersions,
    home: PathBuf,
    projects: Vec<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version : AgactVersions::V1,
            home : PathBuf::new(),
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

    pub fn set_home(&mut self, home: PathBuf) {
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

    pub fn get_homes(&self) -> PathBuf {
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

    pub fn get_to_nots(&self) -> Vec<TimeSinceUniversal> {
        match &self.app_data {
            HomeData::V1(data) => {
                data.to_not.iter().map(TimeSinceUniversal::from).collect()
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
    pub fn add_task(&mut self, task: TasksUniversal) -> String {
        match &mut self.app_data {
            HomeData::V1(data) => {
                let id = gen_id_v1(&data);
                data.tasks.push(task.clone().into());
                id
            }
        }
    }

    pub fn add_to_not(&mut self, task: TimeSinceUniversal) -> String {
        match &mut self.app_data {
            HomeData::V1(data) => {
                let id = gen_id_v1(&data);

                data.to_not.push(task.clone().into());
                id
            }
        }
    }

    pub fn  add_daily(&mut self, task: DailyUniversal) -> String {
        match &mut self.app_data {
            HomeData::V1(data) => {
                let id = gen_id_v1(&data);

                data.daily_tasks.push(task.into());
                id
            }
        }
    }

    pub fn write(&self, path_buf: PathBuf) -> Result<(), toml::ser::Error>{
        fs::write(path_buf.as_path(), toml::to_string(&self.app_data)?).expect("ERROR: Write Failed");
        Ok(())
    }
}