use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TaskHandler: Send + Sync {
    async fn handle(&self, payload: Option<serde_json::Value>) -> Result<()>;
}
