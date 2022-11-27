/*
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * The version of the OpenAPI document: 1.17.3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UserHeatmapData : UserHeatmapData represents the data needed to create a heatmap



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserHeatmapData {
    #[serde(rename = "contributions", skip_serializing_if = "Option::is_none")]
    pub contributions: Option<i64>,
    /// TimeStamp defines a timestamp
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

impl UserHeatmapData {
    /// UserHeatmapData represents the data needed to create a heatmap
    pub fn new() -> UserHeatmapData {
        UserHeatmapData {
            contributions: None,
            timestamp: None,
        }
    }
}


