#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateNotificationGroupRequestDto {
    #[serde(rename = "name")]
    pub name: String,
}

impl CreateNotificationGroupRequestDto {
    pub fn new(name: String) -> CreateNotificationGroupRequestDto {
        CreateNotificationGroupRequestDto { name }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteNotificationGroupResponseDto {
    /// A boolean stating the success of the action
    #[serde(rename = "acknowledged")]
    pub acknowledged: bool,
    /// The status enum for the performed action
    #[serde(rename = "status")]
    pub status: Status,
}

impl DeleteNotificationGroupResponseDto {
    pub fn new(acknowledged: bool, status: Status) -> DeleteNotificationGroupResponseDto {
        DeleteNotificationGroupResponseDto {
            acknowledged,
            status,
        }
    }
}
/// The status enum for the performed action
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "deleted")]
    Deleted,
}

impl Default for Status {
    fn default() -> Status {
        Self::Deleted
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationGroupResponseDto {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "_environmentId")]
    pub _environment_id: String,
    #[serde(rename = "_organizationId")]
    pub _organization_id: String,
    #[serde(rename = "_parentId", skip_serializing_if = "Option::is_none")]
    pub _parent_id: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationGroup {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "_environmentId")]
    pub _environment_id: String,
    #[serde(rename = "_organizationId")]
    pub _organization_id: String,
    #[serde(rename = "_parentId", skip_serializing_if = "Option::is_none")]
    pub _parent_id: Option<String>,
}
