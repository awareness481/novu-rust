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
pub struct CreateTenantRequestDto {
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl CreateTenantRequestDto {
    pub fn new(identifier: String, name: String) -> CreateTenantRequestDto {
        CreateTenantRequestDto {
            identifier,
            name,
            data: None,
        }
    }
}
