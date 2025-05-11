use anyhow::Result;
use dashmap::DashMap;
use log::error;
use std::sync::Arc;
use tokio::time::Duration;
use tokio::time::interval;

use super::TaskHandler;
use super::TaskStorage;
use super::storage::get_storage;
use super::storage::init_storage;
use crate::models::TaskStatus;

pub struct TaskScheduler {
    handlers: DashMap<String, Arc<dyn TaskHandler>>,
    check_interval: Duration,
}

impl TaskScheduler {
    pub fn new(storage: Arc<dyn TaskStorage>, check_interval: Duration) -> Self {
        let _ = init_storage(storage);

        Self {
            handlers: DashMap::new(),
            check_interval,
        }
    }

    pub async fn register_handler(&self, name: impl Into<String>, handler: Arc<dyn TaskHandler>) {
        self.handlers.insert(name.into(), handler);
    }

    pub async fn run(&self) -> Result<()> {
        let mut interval = interval(self.check_interval);

        loop {
            interval.tick().await;
            if let Err(e) = self.process_due_tasks().await {
                error!("Error processing tasks: {}", e);
            }
        }
    }

    async fn process_due_tasks(&self) -> Result<()> {
        let storage = get_storage()?;
        let due_tasks = storage.get_due_tasks(10).await?;

        for task in due_tasks {
            storage
                .update_task_status(task.id, TaskStatus::Processing)
                .await?;

            if let Some(handler) = self.handlers.get(&task.name) {
                match handler.handle().await {
                    Ok(_) => {
                        storage
                            .update_task_status(task.id, TaskStatus::Completed)
                            .await?
                    }
                    Err(e) => {
                        error!("Failed to execute task: {}", e);
                        storage
                            .update_task_status(task.id, TaskStatus::Failed)
                            .await?;
                    }
                }
            }
        }

        Ok(())
    }
}
