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
pub struct OrganizationResponseDto {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "logo", skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(rename = "branding")]
    pub branding: Box<models::OrganizationBrandingResponseDto>,
    #[serde(
        rename = "partnerConfigurations",
        skip_serializing_if = "Option::is_none"
    )]
    pub partner_configurations: Option<Vec<models::IPartnerConfigurationResponseDto>>,
}
