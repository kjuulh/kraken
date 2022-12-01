use std::{
    io::{self, Write},
    process::exit,
};

use async_trait::async_trait;

use super::UI;

pub struct TerminalUI {}

impl TerminalUI {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn query_console(&self) -> eyre::Result<()> {
        print!("Continue? ([Y]es/[N]o): ");
        std::io::stdout().lock().flush()?;
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input)?;

        if input.to_lowercase().starts_with("y") {
            return Ok(());
        } else if input.to_lowercase().starts_with("n") {
            exit(0)
        } else {
            Err(eyre::anyhow!("input not valid"))
        }
    }
}

#[async_trait]
impl UI for TerminalUI {
    async fn confirm(&self) -> eyre::Result<()> {
        match self.query_console().await {
            Ok(_) => Ok(()),
            Err(e) => {
                if e.to_string().starts_with("input not valid") {
                    self.query_console().await
                } else {
                    Err(e)
                }
            }
        }
    }
}
