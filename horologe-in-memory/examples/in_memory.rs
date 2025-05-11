use anyhow::Result;
use chrono::{Duration, Local};
use horologe::scheduler::{Job, TaskHandler, TaskScheduler};
use horologe_in_memory::InMemoryStorage;
use std::sync::Arc;

struct EmailHandler;

#[async_trait::async_trait]
impl TaskHandler for EmailHandler {
    async fn handle(&self) -> Result<()> {
        println!("Sending email");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let storage = Arc::new(InMemoryStorage::new());
    let scheduler = TaskScheduler::new(storage.clone(), std::time::Duration::from_secs(1));

    scheduler
        .register_handler("send_email", Arc::new(EmailHandler))
        .await;

    tokio::spawn(async move {
        scheduler.run().await.expect("Scheduler failed");
    });

    tokio::spawn(async move {
        run_app().await.expect("App failed");
    });

    tokio::time::sleep(std::time::Duration::from_secs(60 * 5)).await;

    Ok(())
}

async fn run_app() -> Result<()> {
    let now = Local::now().naive_local();
    let execution_time = now + Duration::minutes(1);

    Job::name("send_email").at(execution_time).await?;

    Ok(())
}
