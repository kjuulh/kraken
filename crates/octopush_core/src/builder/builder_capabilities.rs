use std::{path::PathBuf, sync::Arc};

use async_trait::async_trait;

use crate::schema::models::Action;

use super::{
    builders::golang_bin::{GolangBinBuild, GolangBinBuildOpts},
    Builder, DynRunnableBin,
};

pub struct BuilderCapabilities;

impl BuilderCapabilities {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Builder for BuilderCapabilities {
    async fn build(&self, action_path: &PathBuf, action: &Action) -> eyre::Result<DynRunnableBin> {
        match action {
            Action::Go { entry } => {
                let bin = GolangBinBuild::new()
                    .build(GolangBinBuildOpts {
                        entry: entry.clone(),
                        src_path: action_path.clone(),
                    })
                    .await?;

                Ok(Arc::new(bin))
            }
        }
    }
}
