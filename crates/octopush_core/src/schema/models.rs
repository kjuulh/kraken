use serde::{Deserialize, Serialize};

pub type Repository = String;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GitPushBranch {
    pub name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GitPushPullRequest {
    pub name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GitPush {
    pub branch: GitPushBranch,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Git {
    pub push: Option<GitPush>,
    pub repositories: Vec<Repository>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GitHubPush {
    #[serde(rename = "pull-request")]
    pub pull_request: GitPushPullRequest,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GiteaPush {
    #[serde(rename = "pull-request")]
    pub pull_request: GitPushPullRequest,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GitHub {
    pub push: Option<GitHubPush>,
    pub repositories: Vec<Repository>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Gitea {
    pub push: Option<GiteaPush>,
    pub repositories: Vec<Repository>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SelectAction {
    pub git: Option<Git>,
    pub github: Option<GitHub>,
    pub gitea: Option<Gitea>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Action {
    #[serde(rename = "go")]
    Go { entry: String },
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "apiVersion")]
pub enum Schema {
    #[serde(rename = "action")]
    Action {
        name: String,
        select: SelectAction,
        action: Action,
    },
}
