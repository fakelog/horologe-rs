use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use horologe::{
    models::{Task, TaskStatus},
    scheduler::TaskStorage,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct InMemoryStorage {
    tasks: Arc<RwLock<HashMap<Uuid, Task>>>,
}

impl Default for InMemoryStorage {
    fn default() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl InMemoryStorage {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl TaskStorage for InMemoryStorage {
    async fn create_task(&self, name: &str, scheduled_at: NaiveDateTime) -> Result<Task> {
        let id = Uuid::new_v4();
        let task = Task {
            id,
            name: name.to_string(),
            scheduled_at,
            status: TaskStatus::Pending,
        };

        let mut tasks = self.tasks.write().await;
        tasks.insert(id, task.clone());

        Ok(task)
    }

    async fn get_due_tasks(&self, limit: u64) -> Result<Vec<Task>> {
        let now = chrono::Local::now().naive_local();
        let tasks = self.tasks.read().await;

        let mut due_tasks: Vec<_> = tasks
            .values()
            .filter(|t| t.status == TaskStatus::Pending && t.scheduled_at <= now)
            .cloned()
            .collect();

        due_tasks.sort_by(|a, b| a.scheduled_at.cmp(&b.scheduled_at));
        due_tasks.truncate(limit as usize);

        Ok(due_tasks)
    }

    async fn update_task_status(&self, task_id: Uuid, status: TaskStatus) -> Result<()> {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(&task_id) {
            task.status = status;
        }
        Ok(())
    }

    async fn remove_task(&self, task_id: Uuid) -> Result<()> {
        let mut tasks = self.tasks.write().await;
        tasks.remove(&task_id);
        Ok(())
    }
}
