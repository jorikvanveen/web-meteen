use chrono::{DateTime, NaiveDateTime, NaiveTime, Offset, TimeZone, Utc};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct Project {
    pub name: String,
    pub project_id: String,
    pub parent_id: Option<String>,
    pub tasks: Vec<Task>,
}

#[wasm_bindgen]
impl Project {
    #[wasm_bindgen(constructor)]
    pub fn new(id: String) -> Self {
        Self {
            name: "".into(),
            project_id: id,
            parent_id: None,
            tasks: vec![],
        }
    }
}

impl From<meteen_model::Project> for Project {
    fn from(value: meteen_model::Project) -> Self {
        Self {
            name: value.name,
            project_id: value.project_id,
            parent_id: value.parent_id,
            tasks: value.tasks.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<Project> for meteen_model::Project {
    fn from(value: Project) -> Self {
        Self {
            name: value.name,
            project_id: value.project_id,
            parent_id: value.parent_id,
            tasks: value.tasks.into_iter().map(Into::into).collect(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug)]
pub struct Task {
    pub task_id: String,
    pub summary: String,
    pub done: bool,
    pub scheduled: Option<DateOrDateTime>,
    pub deadline: Option<DateOrDateTime>,
    pub priority: Priority,
}

#[wasm_bindgen]
impl Task {
    #[wasm_bindgen(constructor)]
    pub fn new(id: String) -> Self {
        Self {
            task_id: id,
            summary: "".into(),
            done: false,
            scheduled: None,
            deadline: None,
            priority: Priority { priority: 1 },
        }
    }
}

impl From<meteen_model::Task> for Task {
    fn from(value: meteen_model::Task) -> Self {
        Self {
            task_id: value.task_id,
            summary: value.summary,
            done: value.done,
            scheduled: value.scheduled.map(Into::into),
            deadline: value.deadline.map(Into::into),
            priority: value.priority.into(),
        }
    }
}

impl From<Task> for meteen_model::Task {
    fn from(value: Task) -> Self {
        Self {
            task_id: value.task_id,
            summary: value.summary,
            done: value.done,
            scheduled: value.scheduled.map(Into::into),
            deadline: value.deadline.map(Into::into),
            priority: value.priority.into(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug)]
pub struct DateOrDateTime {
    pub has_time: bool,
    pub utc_millis: i64,
}

#[wasm_bindgen]
impl DateOrDateTime {
    #[wasm_bindgen(constructor)]
    pub fn new(has_time: bool, utc_millis: i64) -> Self {
        Self {
            has_time,
            utc_millis,
        }
    }
}

impl From<meteen_model::DateOrDateTime> for DateOrDateTime {
    fn from(value: meteen_model::DateOrDateTime) -> Self {
        match value {
            meteen_model::DateOrDateTime::Date(date) => Self {
                has_time: false,
                utc_millis: date
                    .and_hms_opt(12, 0, 0)
                    .unwrap()
                    .and_utc()
                    .timestamp_millis(),
            },
            meteen_model::DateOrDateTime::DateTime(datetime) => Self {
                has_time: true,
                utc_millis: datetime.timestamp_millis(),
            },
        }
    }
}

impl From<DateOrDateTime> for meteen_model::DateOrDateTime {
    fn from(value: DateOrDateTime) -> Self {
        let datetime = DateTime::from_timestamp_millis(value.utc_millis).unwrap();
        if value.has_time {
            meteen_model::DateOrDateTime::DateTime(datetime)
        } else {
            meteen_model::DateOrDateTime::Date(datetime.date_naive())
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug)]
pub struct Priority {
    priority: usize,
}

#[wasm_bindgen]
impl Priority {
    #[wasm_bindgen(constructor)]
    pub fn new(priority: usize) -> Priority {
        Priority { priority }
    }
}

impl From<Priority> for meteen_model::Priority {
    fn from(value: Priority) -> Self {
        match value.priority {
            0 => meteen_model::Priority::Low,
            1 => meteen_model::Priority::Standard,
            2 => meteen_model::Priority::High,
            3 => meteen_model::Priority::Urgent,
            _ => panic!("Invalid priority: {}", value.priority),
        }
    }
}

impl From<meteen_model::Priority> for Priority {
    fn from(value: meteen_model::Priority) -> Self {
        Priority {
            priority: match value {
                meteen_model::Priority::Low => 0,
                meteen_model::Priority::Standard => 1,
                meteen_model::Priority::High => 2,
                meteen_model::Priority::Urgent => 3,
            },
        }
    }
}
