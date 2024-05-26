use crate::shared::PreferenceChannels;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateWorkflowOverrideRequestDto {
    #[serde(rename = "workflowId")]
    pub workflow_id: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "preferenceSettings", skip_serializing_if = "Option::is_none")]
    pub preference_settings: Option<Box<PreferenceChannels>>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateWorkflowOverrideResponseDto {
    #[serde(rename = "_id")]
    pub _id: String,
    #[serde(rename = "_organizationId")]
    pub _organization_id: String,
    #[serde(rename = "_environmentId")]
    pub _environment_id: String,
    #[serde(rename = "_workflowId")]
    pub _workflow_id: String,
    #[serde(rename = "_tenantId")]
    pub _tenant_id: String,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "preferenceSettings")]
    pub preference_settings: Box<PreferenceChannels>,
    #[serde(rename = "deleted")]
    pub deleted: bool,
    #[serde(rename = "deletedAt", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    #[serde(rename = "deletedBy", skip_serializing_if = "Option::is_none")]
    pub deleted_by: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetWorkflowOverrideResponseDto {
    #[serde(rename = "_id")]
    pub _id: String,
    #[serde(rename = "_organizationId")]
    pub _organization_id: String,
    #[serde(rename = "_environmentId")]
    pub _environment_id: String,
    #[serde(rename = "_workflowId")]
    pub _workflow_id: String,
    #[serde(rename = "_tenantId")]
    pub _tenant_id: String,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "preferenceSettings")]
    pub preference_settings: Box<PreferenceChannels>,
    #[serde(rename = "deleted")]
    pub deleted: bool,
    #[serde(rename = "deletedAt", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    #[serde(rename = "deletedBy", skip_serializing_if = "Option::is_none")]
    pub deleted_by: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateWorkflowOverrideRequestDto {
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "preferenceSettings", skip_serializing_if = "Option::is_none")]
    pub preference_settings: Option<Box<PreferenceChannels>>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateWorkflowOverrideResponseDto {
    #[serde(rename = "_id")]
    pub _id: String,
    #[serde(rename = "_organizationId")]
    pub _organization_id: String,
    #[serde(rename = "_environmentId")]
    pub _environment_id: String,
    #[serde(rename = "_workflowId")]
    pub _workflow_id: String,
    #[serde(rename = "_tenantId")]
    pub _tenant_id: String,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "preferenceSettings")]
    pub preference_settings: Box<PreferenceChannels>,
    #[serde(rename = "deleted")]
    pub deleted: bool,
    #[serde(rename = "deletedAt", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    #[serde(rename = "deletedBy", skip_serializing_if = "Option::is_none")]
    pub deleted_by: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}
