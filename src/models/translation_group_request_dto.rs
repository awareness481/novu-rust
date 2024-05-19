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
pub struct TranslationGroupRequestDto {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "locales")]
    pub locales: Vec<Vec<String>>,
}

impl TranslationGroupRequestDto {
    pub fn new(name: String, locales: Vec<Vec<String>>) -> TranslationGroupRequestDto {
        TranslationGroupRequestDto {
            name,
            identifier: None,
            locales,
        }
    }
}
