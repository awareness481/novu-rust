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
pub struct TriggerEventToAllRequestDto {
    /// The trigger identifier associated for the template you wish to send. This identifier can be found on the template page.
    #[serde(rename = "name")]
    pub name: String,
    /// The payload object is used to pass additional custom information that could be used to render the template, or perform routing rules based on it.        This data will also be available when fetching the notifications feed from the API to display certain parts of the UI.
    #[serde(rename = "payload")]
    pub payload: serde_json::Value,
    /// This could be used to override provider specific configurations
    #[serde(rename = "overrides", skip_serializing_if = "Option::is_none")]
    pub overrides: Option<serde_json::Value>,
    /// A unique identifier for this transaction, we will generated a UUID if not provided.
    #[serde(rename = "transactionId", skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(rename = "actor", skip_serializing_if = "Option::is_none")]
    pub actor: Option<Box<models::TriggerEventRequestDtoActor>>,
    #[serde(rename = "tenant", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Box<models::TriggerEventRequestDtoTenant>>,
}
