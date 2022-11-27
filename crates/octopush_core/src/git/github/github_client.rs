use std::sync::Arc;

use async_trait::async_trait;
use octocrab::{Octocrab, OctocrabBuilder};

use super::GitHubClient;

pub struct DefaultGitHubClientOptions {
    pub basicauth: Option<String>,
}

pub struct DefaultGitHubClient {
    github: Arc<Octocrab>,
}

impl DefaultGitHubClient {
    pub fn new(options: &DefaultGitHubClientOptions) -> eyre::Result<Self> {
        let mut github = OctocrabBuilder::new();

        if let Some(basicauth) = options.basicauth.clone() {
            if let Some((username, password)) = basicauth.split_once(":") {
                github = github.basic_auth(username.into(), password.into());
            }
        }

        Ok(Self {
            github: Arc::new(github.build()?),
        })
    }
}

#[async_trait]
impl GitHubClient for DefaultGitHubClient {
    async fn get_clone_url(&self, owner: String, repo_name: String) -> eyre::Result<String> {
        let repo = self.github.repos(&owner, &repo_name).get().await?;
        let clone_url = repo
            .ssh_url
            .ok_or(eyre::anyhow!("clone_url is not set for repository"))?;

        Ok(clone_url)
    }

    async fn create_pull_request(
        &self,
        owner: &String,
        repo_name: &String,
        pull_request_name: &String,
    ) -> eyre::Result<()> {
        self.github
            .pulls(owner, repo_name)
            .create(
                pull_request_name.clone(),
                pull_request_name.to_lowercase().replace(" ", "-"),
                "main",
            )
            .send()
            .await?;

        Ok(())
    }
}
