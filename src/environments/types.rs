#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentResponseDto {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "_organizationId")]
    pub _organization_id: String,
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "apiKeys", skip_serializing_if = "Option::is_none")]
    pub api_keys: Option<Vec<serde_json::Value>>,
    #[serde(rename = "_parentId")]
    pub _parent_id: String,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiKey {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "_userId")]
    pub _user_id: String,
}

impl ApiKey {
    pub fn new(key: String, _user_id: String) -> ApiKey {
        ApiKey { key, _user_id }
    }
}
