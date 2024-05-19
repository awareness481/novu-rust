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
pub struct MessageAction {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "buttons", skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<models::MessageButton>>,
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<models::MessageActionResult>>,
}

impl MessageAction {
    pub fn new() -> MessageAction {
        MessageAction {
            status: None,
            buttons: None,
            result: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "done")]
    Done,
}

impl Default for Status {
    fn default() -> Status {
        Self::Pending
    }
}

