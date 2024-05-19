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
pub struct CreateOrganizationDto {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "logo", skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(rename = "jobTitle", skip_serializing_if = "Option::is_none")]
    pub job_title: Option<JobTitle>,
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "productUseCases", skip_serializing_if = "Option::is_none")]
    pub product_use_cases: Option<serde_json::Value>,
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum JobTitle {
    #[serde(rename = "engineer")]
    Engineer,
    #[serde(rename = "engineering_manager")]
    EngineeringManager,
    #[serde(rename = "architect")]
    Architect,
    #[serde(rename = "product_manager")]
    ProductManager,
    #[serde(rename = "designer")]
    Designer,
    #[serde(rename = "cxo_founder")]
    CxoFounder,
    #[serde(rename = "marketing_manager")]
    MarketingManager,
    #[serde(rename = "other")]
    Other,
}

impl Default for JobTitle {
    fn default() -> JobTitle {
        Self::Engineer
    }
}
