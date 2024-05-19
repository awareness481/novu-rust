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
pub struct DeleteNotificationGroupResponseDto {
    /// A boolean stating the success of the action
    #[serde(rename = "acknowledged")]
    pub acknowledged: bool,
    /// The status enum for the performed action
    #[serde(rename = "status")]
    pub status: Status,
}

impl DeleteNotificationGroupResponseDto {
    pub fn new(acknowledged: bool, status: Status) -> DeleteNotificationGroupResponseDto {
        DeleteNotificationGroupResponseDto {
            acknowledged,
            status,
        }
    }
}
/// The status enum for the performed action
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "deleted")]
    Deleted,
}

impl Default for Status {
    fn default() -> Status {
        Self::Deleted
    }
}
