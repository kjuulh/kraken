use std::{path::PathBuf, process::Stdio};

use eyre::Context;
use tokio::io::{AsyncBufReadExt, BufReader};

pub async fn execute_shell(cmd: String, path: Option<PathBuf>) -> eyre::Result<()> {
    let mut command = tokio::process::Command::new("sh");
    let command = command.arg("-c");

    let command = if let Some(path) = path {
        command.current_dir(path)
    } else {
        command
    };

    let command = command.arg(format!("{}", cmd));

    let command = command.stdout(Stdio::piped());

    let mut child = command.spawn()?;

    let stdout = child
        .stdout
        .take()
        .ok_or(eyre::anyhow!("could not take stdout of command"))?;

    let mut reader = BufReader::new(stdout).lines();

    tokio::spawn(async move {
        let status = child
            .wait()
            .await
            .context(eyre::anyhow!("child process encountered an error"))
            .unwrap();

        if !status.success() {
            tracing::error!(
                cmd,
                status = status.to_string(),
                "child program encountered an error"
            );
        }
    });

    while let Some(line) = reader.next_line().await? {
        tracing::trace!("{}", line)
    }

    Ok(())
}
