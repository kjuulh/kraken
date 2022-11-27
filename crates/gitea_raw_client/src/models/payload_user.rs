/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.17.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PayloadUser : PayloadUser represents the author or committer of a commit



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PayloadUser {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Full name of the commit author
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl PayloadUser {
    /// PayloadUser represents the author or committer of a commit
    pub fn new() -> PayloadUser {
        PayloadUser {
            email: None,
            name: None,
            username: None,
        }
    }
}


