use std::path::PathBuf;

use rand::distributions::{DistString, Standard};

use super::StorageEngine;

pub struct LocalStorageEngine {
    root: PathBuf,
}

impl LocalStorageEngine {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }
}

#[async_trait::async_trait]
impl StorageEngine for LocalStorageEngine {
    async fn allocate_dir(&self) -> eyre::Result<super::TemporaryDir> {
        let subdir_name = Standard.sample_string(&mut rand::thread_rng(), 2);
        let mut path = self.root.clone();
        path.push("tmp");
        path.push(hex::encode(subdir_name));

        Ok(super::TemporaryDir::new(path))
    }

    async fn cleanup(&self) -> eyre::Result<()> {
        let mut path = self.root.clone();
        path.push("tmp");
        tokio::fs::remove_dir_all(path).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::storage::StorageEngine;

    use super::LocalStorageEngine;

    #[tokio::test]
    async fn create_local_storage_engine_and_allocate() {
        let local_storage = LocalStorageEngine::new(PathBuf::new());

        let dir = local_storage.allocate_dir().await.expect("to allocate dir");

        assert_eq!(dir.path().to_string_lossy().len(), 16);
        assert_eq!(dir.path().to_string_lossy().is_empty(), false);
    }
}
