use chrono::NaiveDateTime;
use serde_json::Value;
use std::{fmt, str::FromStr};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub scheduled_at: NaiveDateTime,
    pub status: TaskStatus,
    pub payload: Option<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TaskStatus::Pending => "Pending",
            TaskStatus::Processing => "Processing",
            TaskStatus::Completed => "Completed",
            TaskStatus::Failed => "Failed",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for TaskStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Pending" => Ok(TaskStatus::Pending),
            "Processing" => Ok(TaskStatus::Processing),
            "Completed" => Ok(TaskStatus::Completed),
            "Failed" => Ok(TaskStatus::Failed),
            _ => Err(anyhow::anyhow!("Unknown task status: {}", s)),
        }
    }
}
