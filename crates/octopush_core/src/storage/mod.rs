pub mod local;

use std::{path::PathBuf, sync::Arc};

use async_trait::async_trait;

#[async_trait]
pub trait StorageEngine {
    async fn allocate_dir(&self) -> eyre::Result<TemporaryDir>;
    async fn cleanup(&self) -> eyre::Result<()>;
}

pub type DynStorageEngine = Arc<dyn StorageEngine + Send + Sync>;

#[derive(Clone, Debug)]
pub struct TemporaryDir {
    path: PathBuf,
}

impl TemporaryDir {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn cleanup(self) -> eyre::Result<()> {
        Ok(())
    }
}
