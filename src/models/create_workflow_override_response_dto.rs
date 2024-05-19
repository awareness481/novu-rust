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
pub struct CreateWorkflowOverrideResponseDto {
    #[serde(rename = "_id")]
    pub _id: String,
    #[serde(rename = "_organizationId")]
    pub _organization_id: String,
    #[serde(rename = "_environmentId")]
    pub _environment_id: String,
    #[serde(rename = "_workflowId")]
    pub _workflow_id: String,
    #[serde(rename = "_tenantId")]
    pub _tenant_id: String,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "preferenceSettings")]
    pub preference_settings: Box<models::PreferenceChannels>,
    #[serde(rename = "deleted")]
    pub deleted: bool,
    #[serde(rename = "deletedAt", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    #[serde(rename = "deletedBy", skip_serializing_if = "Option::is_none")]
    pub deleted_by: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl CreateWorkflowOverrideResponseDto {
    pub fn new(_id: String, _organization_id: String, _environment_id: String, _workflow_id: String, _tenant_id: String, active: bool, preference_settings: models::PreferenceChannels, deleted: bool, created_at: String, updated_at: String) -> CreateWorkflowOverrideResponseDto {
        CreateWorkflowOverrideResponseDto {
            _id,
            _organization_id,
            _environment_id,
            _workflow_id,
            _tenant_id,
            active,
            preference_settings: Box::new(preference_settings),
            deleted,
            deleted_at: None,
            deleted_by: None,
            created_at,
            updated_at,
        }
    }
}

