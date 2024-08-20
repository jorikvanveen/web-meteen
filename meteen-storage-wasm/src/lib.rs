mod glue;
mod utils;
use meteen_model::Operation;
use serde::{Deserialize, Serialize};
use std::sync::{LazyLock, Mutex};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, meteen-storage-wasm!");
}

static DB: LazyLock<Mutex<MeteenStorage>> = LazyLock::new(|| Mutex::new(MeteenStorage::new()));

#[wasm_bindgen(getter_with_clone)]
#[derive(Serialize, Deserialize, Clone)]
pub struct MeteenStorage {
    version: u32,
    data: meteen_model::Database,
    unsynced_operations: Vec<Operation>,
}

impl MeteenStorage {
    pub fn new() -> MeteenStorage {
        MeteenStorage {
            version: 0,
            data: meteen_model::Database::new(),
            unsynced_operations: vec![],
        }
    }

    fn apply_operation(&mut self, op: Operation) {
        self.unsynced_operations.push(op.clone());
        self.data.apply_operation(op);
    }
}

#[wasm_bindgen]
pub fn serialize() -> Vec<u8> {
    let db = DB.lock().unwrap();
    bincode::serialize(&*db).unwrap()
}

#[wasm_bindgen]
pub fn deserialize(data: Vec<u8>) {
    let db: MeteenStorage = bincode::deserialize(&data).unwrap();
    *DB.lock().unwrap() = db;
}

#[wasm_bindgen]
pub fn create_task(task: glue::Task, project_id: Option<String>) {
    DB.lock().unwrap().apply_operation(Operation::CreateTask {
        task: task.into(),
        project_id,
    })
}

#[wasm_bindgen]
pub fn get_all_projects() -> JsValue {
    return DB
        .lock()
        .unwrap()
        .data
        .projects
        .values()
        .cloned()
        .map(Into::into)
        .collect::<Vec<glue::Project>>()
        .into();
}

#[wasm_bindgen]
pub fn update_task_done(task_id: String, done: bool) {
    let op = Operation::UpdateTaskDone { task_id, done };
    DB.lock().unwrap().apply_operation(op);
}

#[wasm_bindgen]
pub fn create_new_db() -> MeteenStorage {
    MeteenStorage::new()
}
