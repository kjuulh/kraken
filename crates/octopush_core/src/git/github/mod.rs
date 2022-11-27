pub mod github_client;
pub mod github_provider;

use std::{path::PathBuf, sync::Arc};

use async_trait::async_trait;
use git2::Repository;
use tokio::sync::Mutex;

use crate::schema::models::GitPushPullRequest;

#[async_trait]
pub trait GitHubClient {
    async fn get_clone_url(&self, owner: String, repo_name: String) -> eyre::Result<String>;
    async fn create_pull_request(
        &self,
        owner: &String,
        repo_name: &String,
        pull_request_name: &String,
    ) -> eyre::Result<()>;
}

pub type DynGitHubClient = Arc<dyn GitHubClient + Send + Sync>;

#[async_trait]
pub trait GitHubProvider {
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

pub type DynGitHubProvider = Arc<dyn GitHubProvider + Send + Sync>;
