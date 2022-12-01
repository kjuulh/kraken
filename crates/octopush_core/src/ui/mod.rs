pub mod terminal_ui;

use std::sync::Arc;

use async_trait::async_trait;

#[async_trait]
pub trait UI {
    async fn confirm(&self) -> eyre::Result<()>;
}

pub type DynUI = Arc<dyn UI + Send + Sync>;
