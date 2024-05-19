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
pub struct OverrideResponseDto {
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
