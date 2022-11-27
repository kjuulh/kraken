use std::path::PathBuf;

use crate::builder::DynRunnableBin;

pub struct GolangExecutorOpts {
    pub bin: DynRunnableBin,
    pub victim_path: PathBuf,
}

pub struct GolangExecutor;

impl GolangExecutor {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn execute(&self, opts: GolangExecutorOpts) -> eyre::Result<()> {
        opts.bin.run(&opts.victim_path).await?;

        Ok(())
    }
}
