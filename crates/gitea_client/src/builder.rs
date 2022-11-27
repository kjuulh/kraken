use gitea_raw_client::apis::configuration::{ApiKey, Configuration};

use crate::client::GiteaClient;

pub struct GiteaClientBuilder {
    conf: Configuration,
}

impl GiteaClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_basic_auth(mut self, username: String, password: Option<String>) -> Self {
        self.conf.basic_auth = Some((username, password));
        self
    }

    pub fn set_oauth(mut self, oauth_token: String) -> Self {
        self.conf.oauth_access_token = Some(oauth_token);
        self
    }

    pub fn set_bearer(mut self, bearer_token: String) -> Self {
        self.conf.bearer_access_token = Some(bearer_token);
        self
    }

    pub fn set_api_key(mut self, api_key: String, prefix: Option<String>) -> Self {
        self.conf.api_key = Some(ApiKey {
            key: api_key,
            prefix,
        });
        self
    }

    pub fn set_base_path(mut self, base_path: &String) -> Self {
        self.conf.base_path = base_path.clone();
        self
    }

    pub fn set_client(mut self, client: reqwest::Client) -> Self {
        self.conf.client = client;
        self
    }

    pub fn build(self) -> GiteaClient {
        GiteaClient::new(self.conf)
    }
}

impl Default for GiteaClientBuilder {
    fn default() -> Self {
        Self {
            conf: Configuration::default(),
        }
    }
}

impl From<Configuration> for GiteaClientBuilder {
    fn from(conf: Configuration) -> Self {
        let mut s = Self::default();
        s.conf = conf;
        s
    }
}
