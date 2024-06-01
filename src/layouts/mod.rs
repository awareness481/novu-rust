use std::sync::Arc;
pub mod types;
use crate::client::{Client, ResponseError};

pub use types::*;

pub struct Layouts {
    client: Arc<Client>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateLayoutRequestDto {
    /// User defined custom name and provided by the user that will name the Layout created.
    pub name: String,
    /// User defined custom key that will be a unique identifier for the Layout created.'
    pub identifier: String,
    /// User defined description of the layout
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// User defined content for the layout.
    pub content: String,
    /// User defined variables to render in the layout placeholders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<serde_json::Value>>,
    /// Variable that defines if the layout is chosen as default when creating a layout.
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetLayoutByIdRequestDto {
    #[serde(rename = "layoutId")]
    layout_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OrderBy {
    Ascending = 1,
    Descending = -1,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetLayoutsDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    page_size: Option<u32>,
    #[serde(rename = "sortBy", skip_serializing_if = "Option::is_none")]
    sort_by: Option<String>,
    #[serde(rename = "orderBy", skip_serializing_if = "Option::is_none")]
    order_by: Option<OrderBy>,
}

impl Layouts {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        data: CreateLayoutRequestDto,
    ) -> Result<CreateLayoutResponseDto, ResponseError> {
        self.client.post("/layouts", Some(&data)).await
    }

    pub async fn list(
        self,
        data: GetLayoutsDto,
    ) -> Result<FilterLayoutsResponseDto, ResponseError> {
        match qs::to_string(&data) {
            Ok(query) => {
                if !query.is_empty() {
                    self.client.get(format!("/layouts?{query}")).await
                } else {
                    self.client.get("/layouts").await
                }
            }
            Err(e) => Err(ResponseError::ParseError(e)),
        }
    }

    pub async fn get(
        &self,
        data: GetLayoutByIdRequestDto,
    ) -> Result<GetLayoutResponseDto, ResponseError> {
        self.client
            .get(format!("/layouts/{}", data.layout_id))
            .await
    }

    pub async fn delete(&self, data: GetLayoutByIdRequestDto) -> Result<(), ResponseError> {
        self.client
            .delete(format!("/layouts/{}", data.layout_id))
            .await
    }

    pub async fn update(
        self,
        layout_id: String,
        data: UpdateLayoutRequestDto,
    ) -> Result<UpdateLayoutResponseDto, ResponseError> {
        self.client
            .patch(format!("/layouts/${layout_id}"), Some(&data))
            .await
    }

    pub async fn set_default(&self, data: GetLayoutByIdRequestDto) -> Result<(), ResponseError> {
        self.client
            .get(format!("/layouts/{}/default", data.layout_id))
            .await
    }
}
