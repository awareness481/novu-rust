use crate::{
    client::{Client, Response, ResponseError},
    models::{
        CreateTenantRequestDto, CreateTenantResponseDto, GetTenantResponseDto,
        PaginatedResponseDto, TenantPayloadDto, UpdateTenantResponseDto,
    },
};
use std::sync::Arc;

pub struct Tenants {
    client: Arc<Client>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetTenantsDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl GetTenantsDto {
    pub fn empty(self) -> Self {
        Self {
            page: None,
            limit: None,
        }
    }
}

impl Tenants {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    pub async fn list(
        self,
        data: GetTenantsDto,
    ) -> Result<PaginatedResponseDto<GetTenantResponseDto>, ResponseError> {
        match qs::to_string(&data) {
            Ok(query) => {
                if !query.is_empty() {
                    self.client.get(format!("/tenants?{query}")).await
                } else {
                    self.client.get("/tenants").await
                }
            }
            Err(err) => Err(ResponseError::ParseError(err)),
        }
    }

    pub async fn create(
        self,
        data: CreateTenantRequestDto,
    ) -> Result<CreateTenantResponseDto, ResponseError> {
        self.client.post("/tenants", Some(&data)).await
    }

    pub async fn update(
        self,
        identifier: String,
        data: TenantPayloadDto,
    ) -> Result<UpdateTenantResponseDto, ResponseError> {
        self.client
            .patch(format!("/tenants/{identifier}"), Some(&data))
            .await
    }

    pub async fn delete(self, identifier: String) -> Result<(), ResponseError> {
        self.client.delete(format!("/tenants/{identifier}")).await
    }

    pub async fn get(self, identifier: String) -> Result<GetTenantResponseDto, ResponseError> {
        self.client.get(format!("/tenants/{identifier}")).await
    }
}
