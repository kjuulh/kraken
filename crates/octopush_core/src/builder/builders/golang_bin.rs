use std::path::PathBuf;

use async_trait::async_trait;

use crate::{builder::RunnableBin, shell::execute_shell};

pub struct GolangBinBuildOpts {
    pub entry: String,
    pub src_path: PathBuf,
}

pub struct GolangBinBuild;

impl GolangBinBuild {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn build(&self, opts: GolangBinBuildOpts) -> eyre::Result<GolangBin> {
        tracing::trace!(
            src = opts.src_path.to_string_lossy().to_string(),
            entry = opts.entry,
            "build golang_bin"
        );

        execute_shell(
            format!("go build -o dist/bin {}", opts.entry),
            Some(opts.src_path.clone()),
        )
        .await?;

        let abs_path = std::fs::canonicalize(opts.src_path.join("dist/bin"))?;

        Ok(GolangBin::new(abs_path))
    }
}

pub struct GolangBin {
    path: PathBuf,
}

impl GolangBin {
    fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

#[async_trait]
impl RunnableBin for GolangBin {
    async fn run(&self, victim_path: &PathBuf) -> eyre::Result<()> {
        execute_shell(
            self.path.to_string_lossy().to_string(),
            Some(victim_path.clone()),
        )
        .await?;

        Ok(())
    }
}
