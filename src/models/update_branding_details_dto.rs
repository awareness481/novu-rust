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
pub struct UpdateBrandingDetailsDto {
    #[serde(rename = "logo")]
    pub logo: String,
    #[serde(rename = "color")]
    pub color: String,
    #[serde(rename = "fontColor")]
    pub font_color: String,
    #[serde(rename = "contentBackground")]
    pub content_background: String,
    #[serde(rename = "fontFamily", skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
}

impl UpdateBrandingDetailsDto {
    pub fn new(logo: String, color: String, font_color: String, content_background: String) -> UpdateBrandingDetailsDto {
        UpdateBrandingDetailsDto {
            logo,
            color,
            font_color,
            content_background,
            font_family: None,
        }
    }
}
