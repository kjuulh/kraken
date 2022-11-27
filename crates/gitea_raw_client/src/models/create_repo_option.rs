/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.17.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateRepoOption : CreateRepoOption options when creating repository



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateRepoOption {
    /// Whether the repository should be auto-initialized?
    #[serde(rename = "auto_init", skip_serializing_if = "Option::is_none")]
    pub auto_init: Option<bool>,
    /// DefaultBranch of the repository (used when initializes and in template)
    #[serde(rename = "default_branch", skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<String>,
    /// Description of the repository to create
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Gitignores to use
    #[serde(rename = "gitignores", skip_serializing_if = "Option::is_none")]
    pub gitignores: Option<String>,
    /// Label-Set to use
    #[serde(rename = "issue_labels", skip_serializing_if = "Option::is_none")]
    pub issue_labels: Option<String>,
    /// License to use
    #[serde(rename = "license", skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    /// Name of the repository to create
    #[serde(rename = "name")]
    pub name: String,
    /// Whether the repository is private
    #[serde(rename = "private", skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    /// Readme of the repository to create
    #[serde(rename = "readme", skip_serializing_if = "Option::is_none")]
    pub readme: Option<String>,
    /// Whether the repository is template
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<bool>,
    /// TrustModel of the repository
    #[serde(rename = "trust_model", skip_serializing_if = "Option::is_none")]
    pub trust_model: Option<TrustModel>,
}

impl CreateRepoOption {
    /// CreateRepoOption options when creating repository
    pub fn new(name: String) -> CreateRepoOption {
        CreateRepoOption {
            auto_init: None,
            default_branch: None,
            description: None,
            gitignores: None,
            issue_labels: None,
            license: None,
            name,
            private: None,
            readme: None,
            template: None,
            trust_model: None,
        }
    }
}

/// TrustModel of the repository
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TrustModel {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "collaborator")]
    Collaborator,
    #[serde(rename = "committer")]
    Committer,
    #[serde(rename = "collaboratorcommitter")]
    Collaboratorcommitter,
}

impl Default for TrustModel {
    fn default() -> TrustModel {
        Self::Default
    }
}
