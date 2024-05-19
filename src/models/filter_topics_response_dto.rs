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
pub struct FilterTopicsResponseDto {
    #[serde(rename = "data")]
    pub data: Vec<models::TopicDto>,
    #[serde(rename = "page")]
    pub page: f64,
    #[serde(rename = "pageSize")]
    pub page_size: f64,
    #[serde(rename = "totalCount")]
    pub total_count: f64,
}

impl FilterTopicsResponseDto {
    pub fn new(data: Vec<models::TopicDto>, page: f64, page_size: f64, total_count: f64) -> FilterTopicsResponseDto {
        FilterTopicsResponseDto {
            data,
            page,
            page_size,
            total_count,
        }
    }
}
