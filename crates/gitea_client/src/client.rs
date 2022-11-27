use std::sync::Arc;

use gitea_raw_client::apis::configuration::Configuration;

use crate::apis::{defaults::repository::DefaultRepository, repository::DynRepository};

pub struct GiteaClient {
    repository: DynRepository,
}

impl GiteaClient {
    pub fn new(config: Configuration) -> Self {
        let conf = Arc::new(config);

        Self {
            repository: Arc::new(DefaultRepository::new(conf.clone())),
        }
    }

    pub fn repository(&self) -> DynRepository {
        self.repository.clone()
    }
}

impl From<Configuration> for GiteaClient {
    fn from(conf: Configuration) -> Self {
        Self::new(conf)
    }
}
