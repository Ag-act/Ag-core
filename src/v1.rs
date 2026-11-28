use std::time::SystemTime;
use chrono::{DateTime, Local};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize, Debug, Default)]
pub(crate) struct HomeDataV1 {
    pub daily_tasks: Vec<DailyV1>,
    pub to_not: Vec<TimeSinceV1>,
    pub tasks: Vec<TasksV1>,
}

#[derive(Serialize,Deserialize, Debug)]
pub(crate) struct DailyV1 {
    pub title: String,
    pub description: String,
    pub last_completed: SystemTime,
    pub identifier: String,
}
#[derive(Serialize,Deserialize, Debug)]
pub(crate) struct TimeSinceV1 {
    pub title: String,
    pub description: String,
    pub last_done: SystemTime,
    pub identifier: String,
}
#[derive(Serialize,Deserialize, Debug, Clone)]
pub(crate) struct TasksV1 {
    pub title: String,
    pub description: String,
    pub children: Vec<TasksV1>,
    pub completed: bool,
    pub category : bool,
    pub identifier: String,
}

pub(crate) fn gen_id_v1(data: &HomeDataV1) -> String {
    let mut id = gen_nanoid_v1();
    
    while (data.tasks.iter().any(|t| t.identifier == id) ||
        data.daily_tasks.iter().any(|t| t.identifier == id) ||
        data.to_not.iter().any(|t| t.identifier == id)) {

        id = gen_nanoid_v1();
    }
    
    id
}

pub(crate) fn gen_nanoid_v1() -> String {
    nanoid!(6,&[
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ])
}