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
pub struct BulkSubscriberCreateDto {
    #[serde(rename = "subscribers")]
    pub subscribers: Vec<models::CreateSubscriberRequestDto>,
}

impl BulkSubscriberCreateDto {
    pub fn new(subscribers: Vec<models::CreateSubscriberRequestDto>) -> BulkSubscriberCreateDto {
        BulkSubscriberCreateDto {
            subscribers,
        }
    }
}

