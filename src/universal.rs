use chrono::{DateTime, Local};
use crate::v1::{DailyV1, TasksV1, TimeSinceV1};

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
#[derive(Clone)]
pub struct TimeSinceUniversal {
    title: String,
    description: String,
    identifier: String,
    last_completed: DateTime<Local>,
}
impl TimeSinceUniversal {
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

// V1 Support
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