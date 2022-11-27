use std::sync::Arc;

use octopush_core::{
    builder::{builder_capabilities::BuilderCapabilities, DynBuilder},
    executor::{default_executor::DefaultExecutor, executor::DynExecutor},
    git::{
        git::{LocalGitProvider, LocalGitProviderOptions},
        gitea::{
            client::{DefaultGiteaClient, DefaultGiteaClientOptions},
            provider::DefaultGiteaProvider,
            DynGiteaProvider,
        },
        github::{
            github_client::{DefaultGitHubClient, DefaultGitHubClientOptions},
            github_provider::DefaultGitHubProvider,
            DynGitHubProvider,
        },
        DynGitProvider,
    },
    schema::parser::{DefaultSchemaParser, DynSchemaParser},
    selectors::{
        git_selector::GitSelector, gitea_selector::GiteaSelector, github_selector::GitHubSelector,
    },
    storage::{local::LocalStorageEngine, DynStorageEngine},
};

pub struct ServiceRegister {
    pub storage_engine: DynStorageEngine,
    pub git_provider: DynGitProvider,
    pub schema_parser: DynSchemaParser,
    pub builder: DynBuilder,
    pub executor: DynExecutor,
    pub gitea_provider: DynGiteaProvider,
    pub git_selector: Arc<GitSelector>,
    pub gitea_selector: Arc<GiteaSelector>,
    pub github_provider: DynGitHubProvider,
    pub github_selector: Arc<GitHubSelector>,
}

impl ServiceRegister {
    pub fn new(
        git_provider_options: LocalGitProviderOptions,
        gitea_client_options: DefaultGiteaClientOptions,
        github_client_options: DefaultGitHubClientOptions,
    ) -> eyre::Result<Self> {
        let storage_engine = Arc::new(LocalStorageEngine::new("/tmp/octopush".into()));
        let git_provider = Arc::new(LocalGitProvider::new(
            git_provider_options,
            storage_engine.clone(),
        ));
        let schema_parser = Arc::new(DefaultSchemaParser::new());
        let builder = Arc::new(BuilderCapabilities::new());
        let executor = Arc::new(DefaultExecutor::new(builder.clone()));
        let gitea_client = Arc::new(DefaultGiteaClient::new(&gitea_client_options));
        let gitea_provider = Arc::new(DefaultGiteaProvider::new(
            git_provider.clone(),
            storage_engine.clone(),
            gitea_client.clone(),
        ));
        let git_selector = Arc::new(GitSelector::new(git_provider.clone(), executor.clone()));
        let gitea_selector = Arc::new(GiteaSelector::new(
            gitea_provider.clone(),
            git_provider.clone(),
            executor.clone(),
        ));
        let github_client = Arc::new(DefaultGitHubClient::new(&github_client_options)?);
        let github_provider = Arc::new(DefaultGitHubProvider::new(
            git_provider.clone(),
            storage_engine.clone(),
            github_client.clone(),
        ));
        let github_selector = Arc::new(GitHubSelector::new(
            github_provider.clone(),
            git_provider.clone(),
            executor.clone(),
        ));

        Ok(Self {
            storage_engine,
            git_provider,
            schema_parser,
            builder,
            executor,
            gitea_provider,
            git_selector,
            gitea_selector,
            github_provider,
            github_selector,
        })
    }

    pub async fn cleanup(self) -> eyre::Result<()> {
        self.storage_engine.cleanup().await
    }
}
