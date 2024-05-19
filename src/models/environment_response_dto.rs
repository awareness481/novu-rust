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
pub struct EnvironmentResponseDto {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "_organizationId")]
    pub _organization_id: String,
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "apiKeys", skip_serializing_if = "Option::is_none")]
    pub api_keys: Option<Vec<serde_json::Value>>,
    #[serde(rename = "_parentId")]
    pub _parent_id: String,
}
