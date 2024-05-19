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
pub struct CreateTopicRequestDto {
    /// User defined custom key and provided by the user that will be an unique identifier for the Topic created.
    #[serde(rename = "key")]
    pub key: String,
    /// User defined custom name and provided by the user that will name the Topic created.
    #[serde(rename = "name")]
    pub name: String,
}

impl CreateTopicRequestDto {
    pub fn new(key: String, name: String) -> CreateTopicRequestDto {
        CreateTopicRequestDto { key, name }
    }
}
