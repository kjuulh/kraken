/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.17.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UserSettings : UserSettings represents user settings



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserSettings {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "diff_view_style", skip_serializing_if = "Option::is_none")]
    pub diff_view_style: Option<String>,
    #[serde(rename = "full_name", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(rename = "hide_activity", skip_serializing_if = "Option::is_none")]
    pub hide_activity: Option<bool>,
    /// Privacy
    #[serde(rename = "hide_email", skip_serializing_if = "Option::is_none")]
    pub hide_email: Option<bool>,
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "theme", skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
    #[serde(rename = "website", skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}

impl UserSettings {
    /// UserSettings represents user settings
    pub fn new() -> UserSettings {
        UserSettings {
            description: None,
            diff_view_style: None,
            full_name: None,
            hide_activity: None,
            hide_email: None,
            language: None,
            location: None,
            theme: None,
            website: None,
        }
    }
}


