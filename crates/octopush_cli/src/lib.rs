mod commands;

use clap::Command;

const VERSION: &str = "1.0.0";

#[derive(Debug)]
pub struct OctopushCli {
    cmd: clap::Command,
}

impl OctopushCli {
    pub fn new() -> Self {
        let cmd = Command::new("octopush")
            .version(VERSION)
            .author("Kasper J. Hermansen <contact@kjuulh.io>")
            .about("Your cute action executor")
            .propagate_version(true)
            .subcommand_required(true)
            .subcommand(commands::execute::execute_cmd());

        Self { cmd }
    }

    pub async fn execute(self) -> eyre::Result<()> {
        let matches = self.cmd.get_matches();

        match matches.subcommand() {
            Some(("execute", execute_sub)) => {
                tracing::debug!("executing subcommand 'execute'");
                commands::execute::execute_subcommand(execute_sub).await?;
            }
            Some(_) => return Err(eyre::anyhow!("unknown subcommand, please see --help")),
            None => return Err(eyre::anyhow!("no subcommand specified")),
        }

        Ok(())
    }
}
