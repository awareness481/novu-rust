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
pub struct ChangesResponseDto {
    #[serde(rename = "totalCount")]
    pub total_count: f64,
    #[serde(rename = "data")]
    pub data: Vec<models::ChangeResponseDto>,
    #[serde(rename = "pageSize")]
    pub page_size: f64,
    #[serde(rename = "page")]
    pub page: f64,
}

impl ChangesResponseDto {
    pub fn new(total_count: f64, data: Vec<models::ChangeResponseDto>, page_size: f64, page: f64) -> ChangesResponseDto {
        ChangesResponseDto {
            total_count,
            data,
            page_size,
            page,
        }
    }
}
