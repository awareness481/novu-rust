use crate::messages::types::Channel;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateLayoutResponseDto {
    #[serde(rename = "_id")]
    pub _id: String,
}

impl CreateLayoutResponseDto {
    pub fn new(_id: String) -> CreateLayoutResponseDto {
        CreateLayoutResponseDto { _id }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilterLayoutsResponseDto {
    #[serde(rename = "data")]
    pub data: Vec<LayoutDto>,
    #[serde(rename = "page")]
    pub page: f64,
    #[serde(rename = "pageSize")]
    pub page_size: f64,
    #[serde(rename = "totalCount")]
    pub total_count: f64,
}

impl FilterLayoutsResponseDto {
    pub fn new(
        data: Vec<LayoutDto>,
        page: f64,
        page_size: f64,
        total_count: f64,
    ) -> FilterLayoutsResponseDto {
        FilterLayoutsResponseDto {
            data,
            page,
            page_size,
            total_count,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetLayoutResponseDto {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    #[serde(rename = "_organizationId")]
    pub _organization_id: String,
    #[serde(rename = "_environmentId")]
    pub _environment_id: String,
    #[serde(rename = "_creatorId")]
    pub _creator_id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "channel")]
    pub channel: Channel,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "contentType")]
    pub content_type: String,
    #[serde(rename = "variables", skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<serde_json::Value>>,
    #[serde(rename = "isDefault")]
    pub is_default: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "_parentId", skip_serializing_if = "Option::is_none")]
    pub _parent_id: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateLayoutRequestDto {
    /// User defined custom name and provided by the user that will name the Layout updated.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User defined custom key that will be a unique identifier for the Layout updated.
    #[serde(rename = "identifier")]
    pub identifier: String,
    /// User defined description of the layout
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// User defined content for the layout.
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// User defined variables to render in the layout placeholders.
    #[serde(rename = "variables", skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<serde_json::Value>>,
    /// Variable that defines if the layout is chosen as default when creating a layout.
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateLayoutResponseDto {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    #[serde(rename = "_organizationId")]
    pub _organization_id: String,
    #[serde(rename = "_environmentId")]
    pub _environment_id: String,
    #[serde(rename = "_creatorId")]
    pub _creator_id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "channel")]
    pub channel: Channel,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "contentType")]
    pub content_type: String,
    #[serde(rename = "variables", skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<serde_json::Value>>,
    #[serde(rename = "isDefault")]
    pub is_default: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "_parentId", skip_serializing_if = "Option::is_none")]
    pub _parent_id: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LayoutDto {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    #[serde(rename = "_organizationId")]
    pub _organization_id: String,
    #[serde(rename = "_environmentId")]
    pub _environment_id: String,
    #[serde(rename = "_creatorId")]
    pub _creator_id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "channel")]
    pub channel: Channel,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "contentType")]
    pub content_type: String,
    #[serde(rename = "variables", skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<serde_json::Value>>,
    #[serde(rename = "isDefault")]
    pub is_default: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "_parentId", skip_serializing_if = "Option::is_none")]
    pub _parent_id: Option<String>,
}
