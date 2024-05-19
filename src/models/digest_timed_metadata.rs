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
pub struct DigestTimedMetadata {
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
    #[serde(rename = "digestKey", skip_serializing_if = "Option::is_none")]
    pub digest_key: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "timed", skip_serializing_if = "Option::is_none")]
    pub timed: Option<Box<models::TimedConfig>>,
}

impl DigestTimedMetadata {
    pub fn new(r#type: Type) -> DigestTimedMetadata {
        DigestTimedMetadata {
            amount: None,
            unit: None,
            digest_key: None,
            r#type,
            timed: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Unit {
    #[serde(rename = "seconds")]
    Seconds,
    #[serde(rename = "minutes")]
    Minutes,
    #[serde(rename = "hours")]
    Hours,
    #[serde(rename = "days")]
    Days,
    #[serde(rename = "weeks")]
    Weeks,
    #[serde(rename = "months")]
    Months,
}

impl Default for Unit {
    fn default() -> Unit {
        Self::Seconds
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "timed")]
    Timed,
}

impl Default for Type {
    fn default() -> Type {
        Self::Timed
    }
}
