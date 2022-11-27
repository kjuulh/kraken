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
        DynGitProvider,
    },
    schema::parser::{DefaultSchemaParser, DynSchemaParser},
    selectors::{git_selector::GitSelector, gitea_selector::GiteaSelector},
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
}

impl ServiceRegister {
    pub fn new(
        git_provider_options: LocalGitProviderOptions,
        gitea_client_options: DefaultGiteaClientOptions,
    ) -> Self {
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

        Self {
            storage_engine,
            git_provider,
            schema_parser,
            builder,
            executor,
            gitea_provider,
            git_selector,
            gitea_selector,
        }
    }

    pub async fn cleanup(self) -> eyre::Result<()> {
        self.storage_engine.cleanup().await
    }
}
