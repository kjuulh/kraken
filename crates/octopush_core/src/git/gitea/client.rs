use std::sync::Arc;

use async_trait::async_trait;
use gitea_client::{builder::GiteaClientBuilder, models::CreatePullRequestOption};

use super::GiteaClient;

pub struct DefaultGiteaClientOptions {
    pub url: String,
    pub basicauth: Option<String>,
}

pub struct DefaultGiteaClient {
    gitea_client: Arc<gitea_client::client::GiteaClient>,
}

impl DefaultGiteaClient {
    pub fn new(options: &DefaultGiteaClientOptions) -> Self {
        let mut gitea = GiteaClientBuilder::new().set_base_path(&options.url);

        if let Some(basicauth) = options.basicauth.clone() {
            if let Some((username, password)) = basicauth.split_once(":") {
                gitea = gitea.set_basic_auth(username.into(), Some(password.into()));
            }
        }

        Self {
            gitea_client: Arc::new(gitea.build()),
        }
    }
}

#[async_trait]
impl GiteaClient for DefaultGiteaClient {
    async fn get_clone_url(&self, owner: String, repo_name: String) -> eyre::Result<String> {
        let repo = self
            .gitea_client
            .repository()
            .get(&owner, &repo_name)
            .await?;

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
        self.gitea_client
            .repository()
            .create_pull_request(
                &owner,
                &repo_name,
                Some(CreatePullRequestOption {
                    assignee: None,
                    assignees: None,
                    base: Some("main".into()),
                    body: None,
                    due_date: None,
                    head: Some(pull_request_name.to_lowercase().replace(" ", "-")),
                    labels: None,
                    milestone: None,
                    title: Some(pull_request_name.clone()),
                }),
            )
            .await?;

        Ok(())
    }
}
