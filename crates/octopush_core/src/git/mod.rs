use std::{path::PathBuf, sync::Arc};

use async_trait::async_trait;
use git2::Repository;
use tokio::sync::Mutex;

pub mod git;
pub mod gitea;
pub mod github;

#[async_trait]
pub trait GitProvider {
    async fn clone_from_url(&self, url: &String) -> eyre::Result<(PathBuf, Repository)>;
    async fn create_branch(
        &self,
        repo: Arc<Mutex<Repository>>,
        branch_name: &String,
    ) -> eyre::Result<()>;
    async fn push_branch(
        &self,
        repo: Arc<Mutex<Repository>>,
        branch_name: &String,
    ) -> eyre::Result<()>;
}

pub type DynGitProvider = Arc<dyn GitProvider + Send + Sync>;
