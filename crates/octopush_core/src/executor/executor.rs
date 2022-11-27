use std::{path::PathBuf, sync::Arc};

use async_trait::async_trait;

use crate::schema::models::Action;

#[async_trait]
pub trait Executor {
    async fn execute(
        &self,
        victim_path: &PathBuf,
        action_path: &PathBuf,
        action: &Action,
    ) -> eyre::Result<()>;
}

pub type DynExecutor = Arc<dyn Executor + Send + Sync>;
