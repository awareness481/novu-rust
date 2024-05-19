use crate::{
    client::{Client, ResponseError},
    models::{CreateIntegrationRequestDto, IntegrationResponseDto, UpdateIntegrationRequestDto},
};
use std::sync::Arc;

pub struct Integrations {
    client: Arc<Client>,
}

impl Integrations {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    pub async fn get_all(&self) -> Result<Vec<IntegrationResponseDto>, ResponseError> {
        self.client.get("/integrations").await
    }

    pub async fn get_active(&self) -> Result<Vec<IntegrationResponseDto>, ResponseError> {
        self.client.get("/integrations/active").await
    }

    pub async fn create(
        &self,
        data: CreateIntegrationRequestDto,
    ) -> Result<IntegrationResponseDto, ResponseError> {
        self.client.post("/integrations", Some(&data)).await
    }

    pub async fn update(
        &self,
        integration_id: String,
        data: UpdateIntegrationRequestDto,
    ) -> Result<IntegrationResponseDto, ResponseError> {
        self.client
            .put(format!("/integrations/{integration_id}"), &data)
            .await
    }

    pub async fn set_integration_as_primary(
        &self,
        integration_id: String,
    ) -> Result<IntegrationResponseDto, ResponseError> {
        self.client
            .post(
                format!("/integrations/{integration_id}/set-primary"),
                None::<&bool>,
            )
            .await
    }

    pub async fn delete(
        &self,
        integration_id: String,
    ) -> Result<IntegrationResponseDto, ResponseError> {
        self.client
            .delete(format!("/integrations/{integration_id}"))
            .await
    }

    pub async fn get_webhook_provider_status(
        &self,
        provider_id: String,
    ) -> Result<bool, ResponseError> {
        self.client
            .get(format!(
                "/integrations/webhook/provider/{provider_id}/status"
            ))
            .await
    }
}
