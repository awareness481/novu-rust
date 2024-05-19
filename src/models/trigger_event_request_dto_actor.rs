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

/// TriggerEventRequestDtoActor : It is used to display the Avatar of the provided actor's subscriber id or actor object.     If a new actor object is provided, we will create a new subscriber in our system     
/// It is used to display the Avatar of the provided actor's subscriber id or actor object.     If a new actor object is provided, we will create a new subscriber in our system     
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TriggerEventRequestDtoActor {
    /// Unique identifier of a subscriber in your systems
    String(String),
    SubscriberPayloadDto(Box<models::SubscriberPayloadDto>),
}

impl Default for TriggerEventRequestDtoActor {
    fn default() -> Self {
        Self::String(Default::default())
    }
}
