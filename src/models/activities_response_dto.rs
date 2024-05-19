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
pub struct ActivitiesResponseDto {
    #[serde(rename = "hasMore")]
    pub has_more: bool,
    #[serde(rename = "data")]
    pub data: Vec<models::ActivityNotificationResponseDto>,
    #[serde(rename = "pageSize")]
    pub page_size: f64,
    #[serde(rename = "page")]
    pub page: f64,
}

impl ActivitiesResponseDto {
    pub fn new(
        has_more: bool,
        data: Vec<models::ActivityNotificationResponseDto>,
        page_size: f64,
        page: f64,
    ) -> ActivitiesResponseDto {
        ActivitiesResponseDto {
            has_more,
            data,
            page_size,
            page,
        }
    }
}
