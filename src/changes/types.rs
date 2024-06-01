#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeResponseDto {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    #[serde(rename = "_creatorId")]
    pub _creator_id: String,
    #[serde(rename = "_environmentId")]
    pub _environment_id: String,
    #[serde(rename = "_organizationId")]
    pub _organization_id: String,
    #[serde(rename = "_entityId")]
    pub _entity_id: String,
    pub enabled: bool,
    #[serde(rename = "type")]
    pub r#type: Type,
    pub change: serde_json::Value,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "_parentId", skip_serializing_if = "Option::is_none")]
    pub _parent_id: Option<String>,
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Feed")]
    Feed,
    #[serde(rename = "MessageTemplate")]
    MessageTemplate,
    #[serde(rename = "Layout")]
    Layout,
    #[serde(rename = "DefaultLayout")]
    DefaultLayout,
    #[serde(rename = "NotificationTemplate")]
    NotificationTemplate,
    #[serde(rename = "NotificationGroup")]
    NotificationGroup,
    #[serde(rename = "TranslationGroup")]
    TranslationGroup,
    #[serde(rename = "Translation")]
    Translation,
}

impl Default for Type {
    fn default() -> Type {
        Self::Feed
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeWorkflowStatusRequestDto {
    #[serde(rename = "active")]
    pub active: bool,
}

impl ChangeWorkflowStatusRequestDto {
    pub fn new(active: bool) -> ChangeWorkflowStatusRequestDto {
        ChangeWorkflowStatusRequestDto { active }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangesResponseDto {
    #[serde(rename = "totalCount")]
    pub total_count: f64,
    pub data: Vec<ChangeResponseDto>,
    #[serde(rename = "pageSize")]
    pub page_size: f64,
    #[serde(rename = "page")]
    pub page: f64,
}

impl ChangesResponseDto {
    pub fn new(
        total_count: f64,
        data: Vec<ChangeResponseDto>,
        page_size: f64,
        page: f64,
    ) -> ChangesResponseDto {
        ChangesResponseDto {
            total_count,
            data,
            page_size,
            page,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkApplyChangeDto {
    #[serde(rename = "changeIds")]
    pub change_ids: Vec<String>,
}

impl BulkApplyChangeDto {
    pub fn new(change_ids: Vec<String>) -> BulkApplyChangeDto {
        BulkApplyChangeDto { change_ids }
    }
}
