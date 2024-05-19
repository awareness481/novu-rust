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
pub struct TriggerEventRequestDto {
    /// The trigger identifier of the workflow you wish to send. This identifier can be found on the workflow page.
    #[serde(rename = "name")]
    pub name: String,
    /// The payload object is used to pass additional custom information that could be used to render the workflow, or perform routing rules based on it.        This data will also be available when fetching the notifications feed from the API to display certain parts of the UI.
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
    /// This could be used to override provider specific configurations
    #[serde(rename = "overrides", skip_serializing_if = "Option::is_none")]
    pub overrides: Option<serde_json::Value>,
    /// The recipients list of people who will receive the notification.
    #[serde(rename = "to")]
    pub to: Vec<Vec<String>>,
    /// A unique identifier for this transaction, we will generated a UUID if not provided.
    #[serde(rename = "transactionId", skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(rename = "actor", skip_serializing_if = "Option::is_none")]
    pub actor: Option<Box<models::TriggerEventRequestDtoActor>>,
    #[serde(rename = "tenant", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Box<models::TriggerEventRequestDtoTenant>>,
}

impl TriggerEventRequestDto {
    pub fn new(name: String, to: Vec<Vec<String>>) -> TriggerEventRequestDto {
        TriggerEventRequestDto {
            name,
            payload: None,
            overrides: None,
            to,
            transaction_id: None,
            actor: None,
            tenant: None,
        }
    }
}

