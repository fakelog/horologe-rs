mod entities;

use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::NaiveDateTime;
use entities::tasks::ActiveModel;
use entities::{prelude::Tasks, tasks};
use horologe::{
    models::{Task, TaskStatus},
    scheduler::TaskStorage,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, IntoActiveModel,
    QueryFilter,
};
use sea_orm::{ColumnTrait, QueryOrder, QuerySelect};
use std::str::FromStr;
use uuid::Uuid;

pub struct SeaStorage {
    db: DatabaseConnection,
}

impl SeaStorage {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl TaskStorage for SeaStorage {
    async fn create_task(&self, name: &str, scheduled_at: NaiveDateTime) -> Result<Task> {
        let id = Uuid::now_v7();
        let task = ActiveModel {
            id: Set(id),
            name: Set(name.to_string()),
            scheduled_at: Set(scheduled_at),
            status: Set(TaskStatus::Pending.to_string()),
        };

        let task = task.insert(&self.db).await?;
        Ok(Task {
            id: task.id,
            name: task.name,
            scheduled_at: task.scheduled_at,
            status: TaskStatus::from_str(&task.status)?,
        })
    }

    async fn get_due_tasks(&self, limit: u64) -> Result<Vec<Task>> {
        let now = chrono::Utc::now().naive_local();

        let tasks = Tasks::find()
            .filter(tasks::Column::ScheduledAt.lte(now))
            .filter(tasks::Column::Status.eq(TaskStatus::Pending.to_string()))
            .order_by_asc(tasks::Column::ScheduledAt)
            .limit(limit)
            .all(&self.db)
            .await?;

        Ok(tasks
            .into_iter()
            .map(|t| Task {
                id: t.id,
                name: t.name,
                scheduled_at: t.scheduled_at,
                status: TaskStatus::from_str(&t.status).unwrap_or(TaskStatus::Pending),
            })
            .collect())
    }

    async fn update_task_status(&self, task_id: Uuid, status: TaskStatus) -> Result<()> {
        let mut task = Tasks::find_by_id(task_id)
            .one(&self.db)
            .await?
            .context("Task not found")?
            .into_active_model();

        task.status = Set(status.to_string());
        task.update(&self.db).await?;

        Ok(())
    }

    async fn remove_task(&self, task_id: Uuid) -> Result<()> {
        Tasks::delete_by_id(task_id).exec(&self.db).await?;
        Ok(())
    }
}
