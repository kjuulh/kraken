/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.17.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateFileOptions : CreateFileOptions options for creating files Note: `author` and `committer` are optional (if only one is given, it will be used for the other, otherwise the authenticated user will be used)



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateFileOptions {
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<Box<crate::models::Identity>>,
    /// branch (optional) to base this file from. if not given, the default branch is used
    #[serde(rename = "branch", skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(rename = "committer", skip_serializing_if = "Option::is_none")]
    pub committer: Option<Box<crate::models::Identity>>,
    /// content must be base64 encoded
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "dates", skip_serializing_if = "Option::is_none")]
    pub dates: Option<Box<crate::models::CommitDateOptions>>,
    /// message (optional) for the commit of this file. if not supplied, a default message will be used
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// new_branch (optional) will make a new branch from `branch` before creating the file
    #[serde(rename = "new_branch", skip_serializing_if = "Option::is_none")]
    pub new_branch: Option<String>,
    /// Add a Signed-off-by trailer by the committer at the end of the commit log message.
    #[serde(rename = "signoff", skip_serializing_if = "Option::is_none")]
    pub signoff: Option<bool>,
}

impl CreateFileOptions {
    /// CreateFileOptions options for creating files Note: `author` and `committer` are optional (if only one is given, it will be used for the other, otherwise the authenticated user will be used)
    pub fn new(content: String) -> CreateFileOptions {
        CreateFileOptions {
            author: None,
            branch: None,
            committer: None,
            content,
            dates: None,
            message: None,
            new_branch: None,
            signoff: None,
        }
    }
}


