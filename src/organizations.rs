use crate::{
    client::{Client, ResponseError},
    models::{
        CreateOrganizationDto, MemberResponseDto, OrganizationBrandingResponseDto,
        OrganizationResponseDto, RenameOrganizationDto, UpdateBrandingDetailsDto,
    },
};
use std::sync::Arc;

pub struct Organizations {
    client: Arc<Client>,
}

impl Organizations {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Vec<OrganizationResponseDto>, ResponseError> {
        self.client.get("/organizations").await
    }

    pub async fn create(
        &self,
        data: CreateOrganizationDto,
    ) -> Result<OrganizationResponseDto, ResponseError> {
        self.client.post("/organizations", Some(&data)).await
    }

    pub async fn rename(
        &self,
        data: RenameOrganizationDto,
    ) -> Result<RenameOrganizationDto, ResponseError> {
        self.client.patch("/organizations", Some(&data)).await
    }

    pub async fn get_current(&self) -> Result<OrganizationResponseDto, ResponseError> {
        self.client.get("/organizations/me").await
    }

    pub async fn remove_member(
        &self,
        member_id: String,
    ) -> Result<MemberResponseDto, ResponseError> {
        self.client
            .delete(format!("/organizations/members/${member_id}"))
            .await
    }

    pub async fn get_members(&self) -> Result<MemberResponseDto, ResponseError> {
        self.client.get("/organizations/members").await
    }

    pub async fn update_branding(
        &self,
        data: UpdateBrandingDetailsDto,
    ) -> Result<OrganizationBrandingResponseDto, ResponseError> {
        self.client.put("/organizations/branding", &data).await
    }
}
