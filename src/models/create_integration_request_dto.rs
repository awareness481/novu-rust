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
pub struct CreateIntegrationRequestDto {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "_environmentId", skip_serializing_if = "Option::is_none")]
    pub _environment_id: Option<String>,
    #[serde(rename = "providerId")]
    pub provider_id: String,
    #[serde(rename = "channel")]
    pub channel: Channel,
    #[serde(rename = "credentials", skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Box<models::CredentialsDto>>,
    /// If the integration is active the validation on the credentials field will run
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "check", skip_serializing_if = "Option::is_none")]
    pub check: Option<bool>,
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<models::StepFilter>>,
}

impl CreateIntegrationRequestDto {
    pub fn new(provider_id: String, channel: Channel) -> CreateIntegrationRequestDto {
        CreateIntegrationRequestDto {
            name: None,
            identifier: None,
            _environment_id: None,
            provider_id,
            channel,
            credentials: None,
            active: None,
            check: None,
            conditions: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Channel {
    #[serde(rename = "in_app")]
    InApp,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "chat")]
    Chat,
    #[serde(rename = "push")]
    Push,
}

impl Default for Channel {
    fn default() -> Channel {
        Self::InApp
    }
}
