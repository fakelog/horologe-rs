use anyhow::Result;
use chrono::NaiveDateTime;
use std::borrow::Cow;

use super::storage::get_storage;

pub struct Job<'a> {
    name: Cow<'a, str>,
}

impl<'a> Job<'a> {
    pub fn name(value: impl Into<Cow<'a, str>>) -> Self {
        Self { name: value.into() }
    }

    pub async fn at(&self, time: NaiveDateTime) -> Result<()> {
        let storage = get_storage()?;
        storage.create_task(&self.name, time).await?;
        Ok(())
    }
}
