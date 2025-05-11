use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::NaiveDateTime;
use std::sync::Arc;
use tokio::sync::OnceCell;
use uuid::Uuid;

use crate::models::{Task, TaskStatus};

#[async_trait]
pub trait TaskStorage: Send + Sync {
    async fn create_task(&self, name: &str, scheduled_at: NaiveDateTime) -> Result<Task>;
    async fn get_due_tasks(&self, limit: u64) -> Result<Vec<Task>>;
    async fn update_task_status(&self, task_id: Uuid, status: TaskStatus) -> Result<()>;
    async fn remove_task(&self, task_id: Uuid) -> Result<()>;
}

static TASK_STORAGE: OnceCell<Arc<dyn TaskStorage + Send + Sync>> = OnceCell::const_new();

pub fn get_storage() -> Result<Arc<dyn TaskStorage + Send + Sync>> {
    TASK_STORAGE
        .get()
        .context("Storage not initialized")
        .cloned()
}

pub fn init_storage(storage: Arc<dyn TaskStorage + Send + Sync>) -> Result<()> {
    TASK_STORAGE
        .set(storage)
        .map_err(|_| anyhow::anyhow!("Storage already initialized"))
}
