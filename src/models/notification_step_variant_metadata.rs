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
pub enum NotificationStepVariantMetadata {
    DigestRegularMetadata(Box<models::DigestRegularMetadata>),
    DigestTimedMetadata(Box<models::DigestTimedMetadata>),
    DelayRegularMetadata(Box<models::DelayRegularMetadata>),
    DelayScheduledMetadata(Box<models::DelayScheduledMetadata>),
}

impl Default for NotificationStepVariantMetadata {
    fn default() -> Self {
        Self::DigestRegularMetadata(Default::default())
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
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "backoff")]
    Backoff,
    #[serde(rename = "timed")]
    Timed,
    #[serde(rename = "scheduled")]
    Scheduled,
}

impl Default for Type {
    fn default() -> Type {
        Self::Regular
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BackoffUnit {
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

impl Default for BackoffUnit {
    fn default() -> BackoffUnit {
        Self::Seconds
    }
}
