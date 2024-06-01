#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTenantRequestDto {
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTenantResponseDto {
    #[serde(rename = "_id")]
    pub _id: String,
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(rename = "_environmentId")]
    pub _environment_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetTenantResponseDto {
    #[serde(rename = "_id")]
    pub _id: String,
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(rename = "_environmentId")]
    pub _environment_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TenantPayloadDto {
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl TenantPayloadDto {
    pub fn new(
        identifier: Option<String>,
        name: Option<String>,
        data: Option<serde_json::Value>,
    ) -> TenantPayloadDto {
        TenantPayloadDto {
            identifier,
            name,
            data,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateTenantResponseDto {
    #[serde(rename = "_id")]
    pub _id: String,
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(rename = "_environmentId")]
    pub _environment_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}
