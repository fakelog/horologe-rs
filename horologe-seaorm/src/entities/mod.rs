pub mod prelude;
pub mod tasks;

use horologe::models::{Task, TaskStatus};
use sea_orm::ActiveValue::Set;
use tasks::{ActiveModel, Model};

impl From<Model> for Task {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            scheduled_at: model.scheduled_at,
            status: model.status.parse().unwrap_or(TaskStatus::Pending),
        }
    }
}

impl From<Task> for ActiveModel {
    fn from(task: Task) -> Self {
        Self {
            id: Set(task.id),
            name: Set(task.name),
            scheduled_at: Set(task.scheduled_at),
            status: Set(task.status.to_string()),
        }
    }
}
