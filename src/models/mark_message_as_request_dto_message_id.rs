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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarkMessageAsRequestDtoMessageId {
    String(String),
    Array(Vec<String>),
}

impl Default for MarkMessageAsRequestDtoMessageId {
    fn default() -> Self {
        Self::String(Default::default())
    }
}

