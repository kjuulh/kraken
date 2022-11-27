use std::{path::PathBuf, sync::Arc};

use async_trait::async_trait;
use git2::Repository;
use tokio::sync::Mutex;

use crate::{git::DynGitProvider, schema::models::GitPushPullRequest, storage::DynStorageEngine};

use super::{DynGitHubClient, GitHubProvider};

pub struct DefaultGitHubProvider {
    git_provider: DynGitProvider,
    _storage_engine: DynStorageEngine,
    github_client: DynGitHubClient,
}

impl DefaultGitHubProvider {
    pub fn new(
        git_provider: DynGitProvider,
        storage_engine: DynStorageEngine,
        github_client: DynGitHubClient,
    ) -> Self {
        Self {
            git_provider,
            _storage_engine: storage_engine,
            github_client,
        }
    }
}

#[async_trait]
impl GitHubProvider for DefaultGitHubProvider {
    async fn clone_from_qualified(&self, repo: &String) -> eyre::Result<(PathBuf, Repository)> {
        let (owner, repo_name) = repo
            .split_once("/")
            .ok_or(eyre::anyhow!("repo is not a valid format"))?;

        let clone_url = self
            .github_client
            .get_clone_url(owner.into(), repo_name.into())
            .await?;

        let (path, repo) = self.git_provider.clone_from_url(&clone_url).await?;

        Ok((path, repo))
    }

    async fn create_branch(
        &self,
        repo: Arc<Mutex<Repository>>,
        pull_request: &GitPushPullRequest,
    ) -> eyre::Result<()> {
        tracing::trace!("creating branch");
        self.git_provider
            .create_branch(repo, &pull_request.name)
            .await
    }

    async fn create_pull_request(
        &self,
        repo: Arc<Mutex<Repository>>,
        repo_name: &String,
        pull_request: &GitPushPullRequest,
    ) -> eyre::Result<()> {
        let (owner, repo_name) = repo_name
            .split_once("/")
            .ok_or(eyre::anyhow!("repo is not a valid format"))?;

        tracing::trace!("push_branch");
        self.git_provider
            .push_branch(repo, &pull_request.name)
            .await?;

        tracing::trace!("create_pull_request");
        self.github_client
            .create_pull_request(&owner.into(), &repo_name.into(), &pull_request.name)
            .await
    }
}
