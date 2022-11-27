use dotenv::dotenv;
use eyre::Context;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    if let Err(e) = dotenv().context(".env file not found") {
        tracing::info!(
            error = e.to_string(),
            "no .env file specified, command args are required"
        );
    }

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "octopush,octopush_cli,octopush_core,octopush_infra".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = octopush_cli::OctopushCli::new();
    cli.execute().await
}
