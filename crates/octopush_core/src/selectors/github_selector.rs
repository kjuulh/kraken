use std::{path::PathBuf, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    executor::executor::DynExecutor,
    git::{github::DynGitHubProvider, DynGitProvider},
    schema::models::{Action, GitHub},
};

pub struct GitHubSelector {
    github_provider: DynGitHubProvider,
    git_provider: DynGitProvider,
    executor: DynExecutor,
}

impl GitHubSelector {
    pub fn new(
        github_provider: DynGitHubProvider,
        git_provider: DynGitProvider,
        executor: DynExecutor,
    ) -> Self {
        Self {
            github_provider,
            git_provider,
            executor,
        }
    }

    pub async fn run(
        &self,
        git: &GitHub,
        action_path: &PathBuf,
        action: &Action,
    ) -> eyre::Result<()> {
        tracing::info!("fetching repos");
        for repo in &git.repositories {
            let gp = self.github_provider.clone();
            let (path, repo) = gp.clone_from_qualified(repo).await?;
            let repo = Arc::new(Mutex::new(repo));

            if let Some(push) = &git.push {
                self.git_provider
                    .create_branch(repo.clone(), &push.pull_request.name)
                    .await?;
            }

            self.executor.execute(&path, action_path, action).await?;

            if let Some(push) = &git.push {
                self.git_provider
                    .push_branch(repo, &push.pull_request.name)
                    .await?;
            }
        }

        Ok(())
    }
}
