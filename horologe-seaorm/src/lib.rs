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
    async fn create_task(
        &self,
        name: &str,
        scheduled_at: NaiveDateTime,
        payload: Option<serde_json::Value>,
    ) -> Result<Task> {
        let id = Uuid::now_v7();
        let task = Task {
            id,
            name: name.to_string(),
            scheduled_at,
            status: TaskStatus::Pending,
            payload: payload,
        };

        ActiveModel::from(task.clone()).insert(&self.db).await?;

        Ok(task)
    }

    async fn get_due_tasks(&self, limit: u64) -> Result<Vec<Task>> {
        let now = chrono::Utc::now().naive_utc();

        let tasks = Tasks::find()
            .filter(tasks::Column::ScheduledAt.lte(now))
            .filter(tasks::Column::Status.eq(TaskStatus::Pending.to_string()))
            .order_by_asc(tasks::Column::ScheduledAt)
            .limit(limit)
            .all(&self.db)
            .await?;

        let tasks = tasks.into_iter().map(Task::from).collect();

        Ok(tasks)
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
