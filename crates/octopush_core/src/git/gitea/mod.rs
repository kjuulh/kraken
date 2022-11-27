pub mod client;
pub mod provider;

use std::{path::PathBuf, sync::Arc};

use async_trait::async_trait;
use git2::Repository;
use tokio::sync::Mutex;

use crate::schema::models::GitPushPullRequest;

#[async_trait]
pub trait GiteaClient {
    async fn get_clone_url(&self, owner: String, repo_name: String) -> eyre::Result<String>;
    async fn create_pull_request(
        &self,
        owner: &String,
        repo_name: &String,
        pull_request_name: &String,
    ) -> eyre::Result<()>;
}

pub type DynGiteaClient = Arc<dyn GiteaClient + Send + Sync>;

#[async_trait]
pub trait GiteaProvider {
    async fn clone_from_qualified(&self, repo: &String) -> eyre::Result<(PathBuf, Repository)>;
    async fn create_branch(
        &self,
        repo: Arc<Mutex<Repository>>,
        branch: &GitPushPullRequest,
    ) -> eyre::Result<()>;

    async fn create_pull_request(
        &self,
        repo: Arc<Mutex<Repository>>,
        repo_name: &String,
        pull_request: &GitPushPullRequest,
    ) -> eyre::Result<()>;
}

pub type DynGiteaProvider = Arc<dyn GiteaProvider + Send + Sync>;
