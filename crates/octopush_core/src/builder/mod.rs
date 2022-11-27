pub mod builder_capabilities;
mod builders;

use std::{path::PathBuf, sync::Arc};

use async_trait::async_trait;

use crate::schema::models::Action;

#[async_trait]
pub trait RunnableBin {
    async fn run(&self, victim_path: &PathBuf) -> eyre::Result<()>;
}

pub type DynRunnableBin = Arc<dyn RunnableBin + Send + Sync>;

#[async_trait]
pub trait Builder {
    async fn build(&self, action_path: &PathBuf, action: &Action) -> eyre::Result<DynRunnableBin>;
}

pub type DynBuilder = Arc<dyn Builder + Send + Sync>;
