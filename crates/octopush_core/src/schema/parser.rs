use std::{path::PathBuf, sync::Arc};

use async_trait::async_trait;

use super::models::Schema;

#[async_trait]
pub trait SchemaParser {
    async fn parse_file(&self, file: PathBuf) -> eyre::Result<Schema>;
}

pub type DynSchemaParser = Arc<dyn SchemaParser + Send + Sync>;

#[derive(Debug)]
pub struct DefaultSchemaParser {}

#[async_trait]
impl SchemaParser for DefaultSchemaParser {
    async fn parse_file(&self, file: PathBuf) -> eyre::Result<Schema> {
        let file = tokio::fs::read(file).await?;

        self.parse(file)
    }
}

impl DefaultSchemaParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(&self, contents: Vec<u8>) -> eyre::Result<Schema> {
        let schema = serde_yaml::from_slice(contents.as_slice())?;

        Ok(schema)
    }
}
