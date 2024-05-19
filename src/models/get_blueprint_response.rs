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
pub struct GetBlueprintResponse {
    #[serde(rename = "_id")]
    pub _id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "draft")]
    pub draft: bool,
    #[serde(rename = "preferenceSettings")]
    pub preference_settings: serde_json::Value,
    #[serde(rename = "critical")]
    pub critical: bool,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "steps")]
    pub steps: Vec<serde_json::Value>,
    #[serde(rename = "_organizationId")]
    pub _organization_id: String,
    #[serde(rename = "_creatorId")]
    pub _creator_id: String,
    #[serde(rename = "_environmentId")]
    pub _environment_id: String,
    #[serde(rename = "triggers")]
    pub triggers: Vec<serde_json::Value>,
    #[serde(rename = "_notificationGroupId")]
    pub _notification_group_id: String,
    #[serde(rename = "_parentId", skip_serializing_if = "Option::is_none")]
    pub _parent_id: Option<String>,
    #[serde(rename = "deleted")]
    pub deleted: bool,
    #[serde(rename = "deletedAt")]
    pub deleted_at: String,
    #[serde(rename = "deletedBy")]
    pub deleted_by: String,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "notificationGroup", skip_serializing_if = "Option::is_none")]
    pub notification_group: Option<serde_json::Value>,
    #[serde(rename = "isBlueprint")]
    pub is_blueprint: bool,
    #[serde(rename = "blueprintId", skip_serializing_if = "Option::is_none")]
    pub blueprint_id: Option<String>,
}

impl GetBlueprintResponse {
    pub fn new(_id: String, name: String, description: String, active: bool, draft: bool, preference_settings: serde_json::Value, critical: bool, tags: Vec<String>, steps: Vec<serde_json::Value>, _organization_id: String, _creator_id: String, _environment_id: String, triggers: Vec<serde_json::Value>, _notification_group_id: String, deleted: bool, deleted_at: String, deleted_by: String, is_blueprint: bool) -> GetBlueprintResponse {
        GetBlueprintResponse {
            _id,
            name,
            description,
            active,
            draft,
            preference_settings,
            critical,
            tags,
            steps,
            _organization_id,
            _creator_id,
            _environment_id,
            triggers,
            _notification_group_id,
            _parent_id: None,
            deleted,
            deleted_at,
            deleted_by,
            created_at: None,
            updated_at: None,
            notification_group: None,
            is_blueprint,
            blueprint_id: None,
        }
    }
}

