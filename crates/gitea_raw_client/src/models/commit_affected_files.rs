/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.17.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CommitAffectedFiles : CommitAffectedFiles store information about files affected by the commit



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CommitAffectedFiles {
    #[serde(rename = "filename", skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

impl CommitAffectedFiles {
    /// CommitAffectedFiles store information about files affected by the commit
    pub fn new() -> CommitAffectedFiles {
        CommitAffectedFiles {
            filename: None,
        }
    }
}


