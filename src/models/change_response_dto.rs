/*
 * Novu API
 *
 * Novu REST API. Please see https://docs.novu.co/api-reference for more details.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@novu.co
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeResponseDto {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    #[serde(rename = "_creatorId")]
    pub _creator_id: String,
    #[serde(rename = "_environmentId")]
    pub _environment_id: String,
    #[serde(rename = "_organizationId")]
    pub _organization_id: String,
    #[serde(rename = "_entityId")]
    pub _entity_id: String,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "change")]
    pub change: serde_json::Value,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "_parentId", skip_serializing_if = "Option::is_none")]
    pub _parent_id: Option<String>,
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Feed")]
    Feed,
    #[serde(rename = "MessageTemplate")]
    MessageTemplate,
    #[serde(rename = "Layout")]
    Layout,
    #[serde(rename = "DefaultLayout")]
    DefaultLayout,
    #[serde(rename = "NotificationTemplate")]
    NotificationTemplate,
    #[serde(rename = "NotificationGroup")]
    NotificationGroup,
    #[serde(rename = "TranslationGroup")]
    TranslationGroup,
    #[serde(rename = "Translation")]
    Translation,
}

impl Default for Type {
    fn default() -> Type {
        Self::Feed
    }
}
