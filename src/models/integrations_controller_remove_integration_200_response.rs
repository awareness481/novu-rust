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
pub struct IntegrationsControllerRemoveIntegration200Response {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<models::IntegrationResponseDto>>,
}

impl IntegrationsControllerRemoveIntegration200Response {
    pub fn new() -> IntegrationsControllerRemoveIntegration200Response {
        IntegrationsControllerRemoveIntegration200Response {
            data: None,
        }
    }
}

