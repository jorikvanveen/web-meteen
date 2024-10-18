use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Hash, Clone, PartialEq, Eq, Debug)]
pub struct Project {
    pub name: String,
    pub project_id: String,
    pub parent_id: Option<String>,
    pub tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize, Hash, Clone, PartialEq, Eq, Debug)]
pub struct Task {
    pub task_id: String,
    pub summary: String,
    pub done: bool,
    pub scheduled: Option<DateOrDateTime>,
    pub deadline: Option<DateOrDateTime>,
    pub priority: Priority,
}

#[derive(Serialize, Deserialize, Hash, Clone, PartialEq, Eq, Debug)]
pub enum Priority {
    Low,
    Standard,
    High,
    Urgent,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Operation {
    CreateTask {
        task: Task,
        project_id: Option<String>,
    },
    DeleteTask {
        task_id: String,
    },
    UpdateTaskSummary {
        task_id: String,
        summary: String,
    },
    UpdateTaskDone {
        task_id: String,
        done: bool,
    },
    UpdateTaskScheduled {
        task_id: String,
        scheduled: Option<DateOrDateTime>,
    },
    UpdateTaskDeadline {
        task_id: String,
        deadline: Option<DateOrDateTime>,
    },
    MoveTask {
        task_id: String,
        project_id_to: String,
    },
    CreateProject {
        project: Project,
    },
    DeleteProject {
        project_id: String,
    },
    MoveProject {
        project_id: String,
        parent_id_to: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Hash, Clone, PartialEq, Eq, Debug)]
pub enum DateOrDateTime {
    Date(NaiveDate),
    DateTime(DateTime<Utc>),
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct Database {
    pub projects: HashMap<String, Project>,
}

#[derive(Debug, Error)]
pub enum NotFoundError {
    #[error("The project with id {0} does not exist")]
    NoSuchProject(String),

    #[error("The task with id {0} does not exist")]
    NoSuchTask(String),
}

#[derive(Debug, Error)]
pub enum ApplyError {
    #[error("Tried to access a task or project that does not exist.")]
    NotFound(#[from] NotFoundError),
}

#[derive(Debug, Error)]
pub enum SyncError {
    #[error("The provided operations are incompatible with the state of the database")]
    CouldNotMerge(#[from] ApplyError),
}

impl Database {
    pub fn new() -> Self {
        Self {
            projects: HashMap::from([(
                "inbox".into(),
                Project {
                    name: "Inbox".into(),
                    project_id: "inbox".into(),
                    parent_id: None,
                    tasks: vec![],
                },
            )]),
        }
    }
    pub fn apply_operation(&mut self, operation: Operation) {
        match operation {
            Operation::CreateTask { task, project_id } => match project_id {
                Some(project_id) => match self.projects.get_mut(&project_id) {
                    Some(project) => project.tasks.push(task),
                    None => {
                        tracing::warn!(
                            "Tried to create task in non-existant project: {}",
                            project_id
                        );
                    }
                },
                None => self.projects.get_mut("inbox").unwrap().tasks.push(task), // Unwrap is safe because inbox must exist
            },
            Operation::DeleteTask { task_id } => {
                let matches_id = |task: &Task| task.task_id == task_id;

                for project in self.projects.values_mut() {
                    if let Some(index) = project.tasks.iter().position(matches_id) {
                        project.tasks.remove(index);
                    }
                }

                tracing::warn!("Tried to delete non-existant task: {}", task_id);
            }
            Operation::UpdateTaskSummary { task_id, summary } => {
                let task = self.get_task_mut(&task_id);

                if let Some(task) = task {
                    task.summary = summary;
                } else {
                    tracing::warn!("Tried to update summary of non-existant task: {}", task_id);
                }
            }
            Operation::UpdateTaskDone { task_id, done } => {
                let task = self.get_task_mut(&task_id);

                if let Some(task) = task {
                    task.done = done;
                } else {
                    tracing::warn!(
                        "Tried to update done status of non-existant task: {}",
                        task_id
                    );
                }
            }
            Operation::UpdateTaskScheduled { task_id, scheduled } => {
                let task = self.get_task_mut(&task_id);

                if let Some(task) = task {
                    task.scheduled = scheduled;
                } else {
                    tracing::warn!(
                        "Tried to update scheduled date of non-existant task: {}",
                        task_id
                    );
                }
            }
            Operation::UpdateTaskDeadline { task_id, deadline } => {
                let task = self.get_task_mut(&task_id);

                if let Some(task) = task {
                    task.deadline = deadline;
                } else {
                    tracing::warn!("Tried to update deadline of non-existant task: {}", task_id);
                }
            }
            Operation::MoveTask {
                task_id,
                project_id_to,
            } => {
                if !self.projects.contains_key(&project_id_to) {
                    tracing::warn!("Tried to move task to non-existant project: {}", task_id);
                    return;
                }

                let task = {
                    let old_project = match self.project_of_mut(&task_id) {
                        Ok(old_project) => old_project,
                        Err(_) => {
                            tracing::warn!("Tried to move non-existant task: {}", task_id);
                            return;
                        }
                    };

                    let index = old_project
                        .tasks
                        .iter()
                        .position(|task| task.task_id == task_id)
                        .unwrap(); // Unwrap is safe because project_of_mut implies that the project contains this task

                    old_project.tasks.remove(index)
                };

                // Unwrap is safe because we check if the project exists at the top of this case
                let new_project = self.projects.get_mut(&project_id_to).unwrap();

                new_project.tasks.push(task);
            }
            Operation::CreateProject { project } => {
                let id = project.project_id.clone();
                self.projects.insert(id, project);
            }
            Operation::DeleteProject { project_id } => {
                match self.projects.remove(&project_id) {
                    Some(_) => {}
                    None => {
                        tracing::warn!("Tried to remove non-existant project: {}", project_id);
                    }
                };
            }
            Operation::MoveProject {
                project_id,
                parent_id_to,
            } => {
                match parent_id_to {
                    Some(ref id) => {
                        if !self.projects.contains_key(id) {
                            tracing::warn!(
                                "Tried to move project into non-existant project: {:?}",
                                parent_id_to
                            );
                            return;
                        }
                    }
                    None => {}
                }

                let project = match self.projects.get_mut(&project_id) {
                    Some(project) => project,
                    None => {
                        tracing::warn!("Tried to move non-existant project: {:?}", project_id);
                        return;
                    }
                };

                project.parent_id = parent_id_to;
            }
        }
    }

    pub fn batch_operations(&mut self, ops: Vec<Operation>) {
        for op in ops {
            self.apply_operation(op);
        }
    }

    pub fn all_tasks<'a>(&'a self) -> Vec<&'a Task> {
        self.projects
            .values()
            .flat_map(|proj| &proj.tasks)
            .collect()
    }

    pub fn all_tasks_mut<'a>(&'a mut self) -> Vec<&'a mut Task> {
        self.projects
            .values_mut()
            .flat_map(|proj| &mut proj.tasks)
            .collect()
    }

    pub fn get_task<'a>(&'a self, task_id: &str) -> Option<&'a Task> {
        self.all_tasks()
            .into_iter()
            .find(|task| task.task_id == task_id)
    }

    pub fn get_task_mut<'a>(&'a mut self, task_id: &str) -> Option<&'a mut Task> {
        self.all_tasks_mut()
            .into_iter()
            .find(|task| task.task_id == task_id)
    }

    pub fn project_of(&self, task_id: &str) -> Result<&Project, NotFoundError> {
        for project in self.projects.values() {
            if project
                .tasks
                .iter()
                .find(|task| task.task_id == task_id)
                .is_some()
            {
                return Ok(project);
            }
        }

        return Err(NotFoundError::NoSuchTask(task_id.into())); // Every task must be in a project
    }

    pub fn project_of_mut(&mut self, task_id: &str) -> Result<&mut Project, NotFoundError> {
        for project in self.projects.values_mut() {
            if project
                .tasks
                .iter()
                .find(|task| task.task_id == task_id)
                .is_some()
            {
                return Ok(project);
            }
        }

        return Err(NotFoundError::NoSuchTask(task_id.into())); // Every task must be in a project
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{Database, Operation, Priority, Project, Task};

    #[test]
    pub fn sync() {
        let mut server_state = Database::new();
        server_state.batch_operations(vec![
            Operation::CreateProject {
                project: Project {
                    name: "My Project".into(),
                    project_id: "myproj".into(),
                    parent_id: None,
                    tasks: vec![],
                },
            },
            Operation::CreateTask {
                task: Task {
                    task_id: "mytask".into(),
                    summary: "My Epic Task".into(),
                    done: false,
                    scheduled: None,
                    deadline: None,
                    priority: Priority::Standard,
                },
                project_id: Some("myproj".into()),
            },
            Operation::CreateTask {
                task: Task {
                    task_id: "mysecondtask".into(),
                    summary: "My Epic Second Task".into(),
                    done: true,
                    scheduled: None,
                    deadline: None,
                    priority: Priority::Standard,
                },
                project_id: Some("myproj".into()),
            },
        ]);

        let mut client_a_state = server_state.clone();
        let mut client_b_state = server_state.clone();

        let client_a_ops = Vec::from([Operation::UpdateTaskDone {
            task_id: "mytask".into(),
            done: true,
        }]);

        let client_b_ops = Vec::from([
            Operation::UpdateTaskDone {
                task_id: "mysecondtask".into(),
                done: false,
            },
            Operation::UpdateTaskSummary {
                task_id: "mytask".into(),
                summary: "The first task".into(),
            },
        ]);

        // Both clients do offline operations
        client_a_state.batch_operations(client_a_ops.clone());
        client_b_state.batch_operations(client_b_ops.clone());

        // Both of the clients sync
        server_state.batch_operations(client_a_ops.clone());
        server_state.batch_operations(client_b_ops.clone());
        client_a_state.batch_operations(client_b_ops);
        client_b_state.batch_operations(client_a_ops);

        // After syncing all states should be equal
        assert_eq!(server_state, client_a_state);
        assert_eq!(server_state, client_b_state);
    }
}
