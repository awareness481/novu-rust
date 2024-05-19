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
pub struct EmailBlock {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "styles", skip_serializing_if = "Option::is_none")]
    pub styles: Option<Box<models::EmailBlockStyles>>,
}

impl EmailBlock {
    pub fn new(r#type: Type, content: String) -> EmailBlock {
        EmailBlock {
            r#type,
            content,
            url: None,
            styles: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "button")]
    Button,
}

impl Default for Type {
    fn default() -> Type {
        Self::Text
    }
}
