#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateFeedRequestDto {
    #[serde(rename = "name")]
    pub name: String,
}

impl CreateFeedRequestDto {
    pub fn new(name: String) -> CreateFeedRequestDto {
        CreateFeedRequestDto { name }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeedResponseDto {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "_environmentId")]
    pub _environment_id: String,
    #[serde(rename = "_organizationId")]
    pub _organization_id: String,
}
