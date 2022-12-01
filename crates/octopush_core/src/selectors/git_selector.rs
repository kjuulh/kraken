use std::{path::PathBuf, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    executor::executor::DynExecutor,
    git::DynGitProvider,
    schema::models::{Action, Git},
};

pub struct GitSelector {
    git_provider: DynGitProvider,
    executor: DynExecutor,
}

impl GitSelector {
    pub fn new(git_provider: DynGitProvider, executor: DynExecutor) -> Self {
        Self {
            git_provider,
            executor,
        }
    }

    pub async fn run(
        &self,
        git: &Git,
        action_path: &PathBuf,
        action: &Action,
        dryrun: bool,
    ) -> eyre::Result<()> {
        tracing::info!("fetching repos");
        for repo in &git.repositories {
            let gp = self.git_provider.clone();
            let (path, repo) = gp.clone_from_url(repo).await?;
            let repo = Arc::new(Mutex::new(repo));

            if let Some(push) = &git.push {
                self.git_provider
                    .create_branch(repo.clone(), &push.branch.name)
                    .await?;
            }

            self.executor.execute(&path, action_path, action).await?;

            if dryrun {
                return Ok(());
            }

            if let Some(push) = &git.push {
                self.git_provider
                    .push_branch(repo, &push.branch.name)
                    .await?;
            }
        }

        Ok(())
    }
}
