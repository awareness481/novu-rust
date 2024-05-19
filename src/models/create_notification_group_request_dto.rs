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
pub struct CreateNotificationGroupRequestDto {
    #[serde(rename = "name")]
    pub name: String,
}

impl CreateNotificationGroupRequestDto {
    pub fn new(name: String) -> CreateNotificationGroupRequestDto {
        CreateNotificationGroupRequestDto { name }
    }
}
