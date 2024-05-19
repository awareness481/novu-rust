use crate::{
    client::{Client, ResponseError},
    models::{ApiKey, EnvironmentResponseDto},
};
use std::sync::Arc;

pub struct Environments {
    client: Arc<Client>,
}

impl Environments {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    pub async fn get_current(self) -> Result<EnvironmentResponseDto, ResponseError> {
        self.client.get("/environments/me").await
    }

    pub async fn get_all(self) -> Result<Vec<EnvironmentResponseDto>, ResponseError> {
        self.client.get("/environments").await
    }

    pub async fn get_api_keys(self) -> Result<Vec<ApiKey>, ResponseError> {
        self.client.get("/environments/api-keys").await
    }

    pub async fn regenerate_api_keys(self) -> Result<Vec<ApiKey>, ResponseError> {
        self.client
            .post::<Vec<ApiKey>>("/environments/api-keys/regenerate", None::<&bool>)
            .await
    }
    // deprecated
    // pub async fn create(payload: crea) {
    //     self.client.post("environments", payload)
    // }
}
