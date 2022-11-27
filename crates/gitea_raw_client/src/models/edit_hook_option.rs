/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.17.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EditHookOption : EditHookOption options when modify one hook



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EditHookOption {
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "branch_filter", skip_serializing_if = "Option::is_none")]
    pub branch_filter: Option<String>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
}

impl EditHookOption {
    /// EditHookOption options when modify one hook
    pub fn new() -> EditHookOption {
        EditHookOption {
            active: None,
            branch_filter: None,
            config: None,
            events: None,
        }
    }
}

