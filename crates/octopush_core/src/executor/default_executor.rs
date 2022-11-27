use std::path::PathBuf;

use async_trait::async_trait;

use crate::{builder::DynBuilder, schema::models::Action};

use super::{
    executor::Executor,
    executors::golang::{GolangExecutor, GolangExecutorOpts},
};

pub struct DefaultExecutor {
    builder: DynBuilder,
}

impl DefaultExecutor {
    pub fn new(builder: DynBuilder) -> Self {
        Self { builder }
    }
}

#[async_trait]
impl Executor for DefaultExecutor {
    async fn execute(
        &self,
        victim_path: &PathBuf,
        action_path: &PathBuf,
        action: &Action,
    ) -> eyre::Result<()> {
        tracing::trace!(
            victim_path = victim_path.to_string_lossy().to_string(),
            "execute"
        );
        let bin = self.builder.build(action_path, action).await?;
        match action {
            Action::Go { .. } => {
                GolangExecutor::new()
                    .execute(GolangExecutorOpts {
                        bin,
                        victim_path: victim_path.clone(),
                    })
                    .await?
            }
        }

        Ok(())
    }
}
