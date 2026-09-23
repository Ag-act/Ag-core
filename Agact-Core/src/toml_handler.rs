use std::fs;
use std::path::PathBuf;
use chrono::{DateTime, Local};
use serde::Deserialize;

// Toml Data V1
#[derive(Deserialize, Debug)]
pub struct TOMLdataV1 {
    daily_tasks: Vec<DailyTask>,
    to_not: Vec<ToNot>,
    tasks: Vec<Tasks>,
}

impl Default for TOMLdataV1 {
    fn default() -> Self {
        Self {daily_tasks: vec![], to_not: vec![], tasks: vec![]}
    }
}

#[derive(Deserialize, Debug)]
pub struct DailyTask {
    title: String,
    description: String,
    last_completed: DateTime<Local>,
}

#[derive(Deserialize, Debug)]
pub struct ToNot {
    title: String,
    description: String,
    last_done: DateTime<Local>,
}

#[derive(Deserialize, Debug)]
pub struct Tasks {
    id : String,
    title: String,
    description: String,
    children: Vec<Tasks>,
    completed: bool,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "version", rename_all = "kebab-case")]
pub enum AppData {
    #[serde(rename = "1")]
    V1(TOMLdataV1)
}

pub struct Data {
    pub app_data: AppData
}

impl Data {
    fn new(data : String) -> Data {
        Data {
            app_data: {
                toml::from_str(&data).unwrap()
            }
        }
    }

    fn get_daily_tasks(&self) -> &Vec<DailyTask> {
        match &self.app_data {
            AppData::V1(data) => {
                &data.daily_tasks
            }
        }
    }

    fn get_to_nots(&self) -> &Vec<ToNot> {
        match &self.app_data {
            AppData::V1(data) => {
                &data.to_not
            }
        }
    }

    fn get_tasks(&self) -> &Vec<Tasks> {
        match &self.app_data {
            AppData::V1(data) => {
                &data.tasks
            }
        }
    }
}