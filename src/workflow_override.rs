use crate::{
    client::{Client, ResponseError},
    models::{
        CreateWorkflowOverrideRequestDto, CreateWorkflowOverrideResponseDto,
        GetWorkflowOverrideResponseDto, UpdateWorkflowOverrideRequestDto,
        UpdateWorkflowOverrideResponseDto,
    },
};
use std::{fmt::format, sync::Arc};

pub struct WorkflowOverride {
    client: Arc<Client>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetWorkflowOverrides {
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl WorkflowOverride {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    pub async fn update_one_by_id(
        self,
        override_id: String,
        data: UpdateWorkflowOverrideRequestDto,
    ) -> Result<UpdateWorkflowOverrideResponseDto, ResponseError> {
        self.client
            .put(format!("/workflow-overrides/{override_id}"), &data)
            .await
    }

    pub async fn create(
        self,
        data: CreateWorkflowOverrideRequestDto,
    ) -> Result<CreateWorkflowOverrideResponseDto, ResponseError> {
        self.client.post("/workflow-overrides", Some(&data)).await
    }

    pub async fn get_one_by_id(
        self,
        override_id: String,
    ) -> Result<GetWorkflowOverrideResponseDto, ResponseError> {
        self.client
            .get(format!("/workflow-overrides/{override_id}"))
            .await
    }

    pub async fn list(
        self,
        data: GetWorkflowOverrides,
    ) -> Result<GetWorkflowOverrideResponseDto, ResponseError> {
        match qs::to_string(&data) {
            Ok(query) => {
                if !query.is_empty() {
                    self.client
                        .get(format!("/workflow-overrides?{query}"))
                        .await
                } else {
                    self.client.get("/workflow-overrides").await
                }
            }
            Err(err) => Err(ResponseError::ParseError(err)),
        }
    }

    pub async fn get_one_by_tenant_id_and_workflow_id(
        self,
        tenant_id: String,
        workflow_id: String,
    ) -> Result<GetWorkflowOverrideResponseDto, ResponseError> {
        self.client
            .get(format!(
                "/workflow-overrides/workflows/${workflow_id}/tenants/${tenant_id}"
            ))
            .await
    }

    pub async fn update_one_by_tenant_id_and_workflow_id(
        self,
        tenant_id: String,
        workflow_id: String,
        data: UpdateWorkflowOverrideRequestDto,
    ) -> Result<UpdateWorkflowOverrideResponseDto, ResponseError> {
        self.client
            .put(
                format!("/workflow-overrides/workflows/${workflow_id}/tenants/${tenant_id}"),
                &data,
            )
            .await
    }

    pub async fn delete(self, override_id: String) -> Result<(), ResponseError> {
        self.client
            .delete(format!("/workflow-overrides/{override_id}"))
            .await
    }
}
