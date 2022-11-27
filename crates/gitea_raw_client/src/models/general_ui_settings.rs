/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.17.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GeneralUiSettings : GeneralUISettings contains global ui settings exposed by API



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GeneralUiSettings {
    #[serde(rename = "allowed_reactions", skip_serializing_if = "Option::is_none")]
    pub allowed_reactions: Option<Vec<String>>,
    #[serde(rename = "custom_emojis", skip_serializing_if = "Option::is_none")]
    pub custom_emojis: Option<Vec<String>>,
    #[serde(rename = "default_theme", skip_serializing_if = "Option::is_none")]
    pub default_theme: Option<String>,
}

impl GeneralUiSettings {
    /// GeneralUISettings contains global ui settings exposed by API
    pub fn new() -> GeneralUiSettings {
        GeneralUiSettings {
            allowed_reactions: None,
            custom_emojis: None,
            default_theme: None,
        }
    }
}


