use std::path::PathBuf;

use clap::{Arg, ArgAction, ArgMatches, Command};
use octopush_core::{
    git::{
        git::LocalGitProviderOptions, gitea::client::DefaultGiteaClientOptions,
        github::github_client::DefaultGitHubClientOptions,
    },
    schema,
};
use octopush_infra::service_register::ServiceRegister;

pub fn execute_cmd() -> Command {
    Command::new("execute")
        .about("execute a certain action")
        .arg(
            Arg::new("action")
                .long("action")
                .short('a')
                .action(ArgAction::Set)
                .help("action path to your local octopush.yaml file")
                .long_help("action path to your local octopush.yaml file")
                .default_value(".")
                .required(true),
        )
        .arg(
            Arg::new("gitea-api-token")
                .long("gitea-api-token")
                .action(ArgAction::Set)
                .env("GITEA_API_TOKEN")
                .required(false),
        )
        .arg(
            Arg::new("gitea-username")
                .long("gitea-username")
                .action(ArgAction::Set)
                .env("GITEA_USERNAME")
                .required(false),
        )
        .arg(
            Arg::new("gitea-url")
                .long("gitea-url")
                .action(ArgAction::Set)
                .env("GITEA_URL")
                .required(false),
        )
        .arg(
            Arg::new("github-api-token")
                .long("github-api-token")
                .action(ArgAction::Set)
                .env("GITHUB_API_TOKEN")
                .required(false),
        )
        .arg(
            Arg::new("github-username")
                .long("github-username")
                .action(ArgAction::Set)
                .env("GITHUB_USERNAME")
                .required(false),
        )
}

pub async fn execute_subcommand(args: &ArgMatches) -> eyre::Result<()> {
    let action = args
        .get_one::<String>("action")
        .ok_or(eyre::anyhow!("--action is required"))?;

    let gitea_http_token = args.get_one::<String>("gitea-api-token");
    let gitea_username = args.get_one::<String>("gitea-username");
    let gitea_url = args.get_one::<String>("gitea-url");

    let github_http_token = args.get_one::<String>("github-api-token");
    let github_username = args.get_one::<String>("github-username");

    let service_register = ServiceRegister::new(
        LocalGitProviderOptions { http_auth: None },
        DefaultGiteaClientOptions {
            url: gitea_url.map(|g| g.clone()).unwrap_or("".into()),
            basicauth: gitea_username
                .zip(gitea_http_token)
                .map(|(u, ht)| format!("{}:{}", u, ht))
                .map(|t| t.clone()),
        },
        DefaultGitHubClientOptions {
            basicauth: github_username
                .zip(github_http_token)
                .map(|(u, ht)| format!("{}:{}", u, ht))
                .map(|t| t.clone()),
        },
    )?;

    let action_path: PathBuf = action.into();

    let schema = service_register
        .schema_parser
        .parse_file(action_path.join("octopush.yml"))
        .await?;

    match schema {
        schema::models::Schema::Action {
            name,
            select,
            action,
        } => {
            tracing::debug!(name, "running action");

            if let Some(git) = &select.git {
                service_register
                    .git_selector
                    .run(git, &action_path, &action)
                    .await?;
            }

            if let Some(gitea) = &select.gitea {
                service_register
                    .gitea_selector
                    .run(gitea, &action_path, &action)
                    .await?;
            }

            if let Some(github) = &select.github {
                service_register
                    .github_selector
                    .run(github, &action_path, &action)
                    .await?;
            }
        }
    }

    service_register.cleanup().await?;

    Ok(())
}
