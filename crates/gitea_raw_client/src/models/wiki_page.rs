/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.17.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WikiPage : WikiPage a wiki page



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WikiPage {
    #[serde(rename = "commit_count", skip_serializing_if = "Option::is_none")]
    pub commit_count: Option<i64>,
    /// Page content, base64 encoded
    #[serde(rename = "content_base64", skip_serializing_if = "Option::is_none")]
    pub content_base64: Option<String>,
    #[serde(rename = "footer", skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    #[serde(rename = "html_url", skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(rename = "last_commit", skip_serializing_if = "Option::is_none")]
    pub last_commit: Option<Box<crate::models::WikiCommit>>,
    #[serde(rename = "sidebar", skip_serializing_if = "Option::is_none")]
    pub sidebar: Option<String>,
    #[serde(rename = "sub_url", skip_serializing_if = "Option::is_none")]
    pub sub_url: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl WikiPage {
    /// WikiPage a wiki page
    pub fn new() -> WikiPage {
        WikiPage {
            commit_count: None,
            content_base64: None,
            footer: None,
            html_url: None,
            last_commit: None,
            sidebar: None,
            sub_url: None,
            title: None,
        }
    }
}


