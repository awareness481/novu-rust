#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTopicRequestDto {
    /// User defined custom key and provided by the user that will be an unique identifier for the Topic created.
    #[serde(rename = "key")]
    pub key: String,
    /// User defined custom name and provided by the user that will name the Topic created.
    #[serde(rename = "name")]
    pub name: String,
}

impl CreateTopicRequestDto {
    pub fn new(key: String, name: String) -> CreateTopicRequestDto {
        CreateTopicRequestDto { key, name }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilterTopicsResponseDto {
    #[serde(rename = "data")]
    pub data: Vec<TopicDto>,
    #[serde(rename = "page")]
    pub page: f64,
    #[serde(rename = "pageSize")]
    pub page_size: f64,
    #[serde(rename = "totalCount")]
    pub total_count: f64,
}

impl FilterTopicsResponseDto {
    pub fn new(
        data: Vec<TopicDto>,
        page: f64,
        page_size: f64,
        total_count: f64,
    ) -> FilterTopicsResponseDto {
        FilterTopicsResponseDto {
            data,
            page,
            page_size,
            total_count,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetTopicResponseDto {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    #[serde(rename = "_organizationId")]
    pub _organization_id: String,
    #[serde(rename = "_environmentId")]
    pub _environment_id: String,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "subscribers")]
    pub subscribers: Vec<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenameTopicRequestDto {
    /// User defined custom name and provided by the user to rename the topic.
    #[serde(rename = "name")]
    pub name: String,
}

impl RenameTopicRequestDto {
    pub fn new(name: String) -> RenameTopicRequestDto {
        RenameTopicRequestDto { name }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenameTopicResponseDto {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    #[serde(rename = "_organizationId")]
    pub _organization_id: String,
    #[serde(rename = "_environmentId")]
    pub _environment_id: String,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "subscribers")]
    pub subscribers: Vec<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopicPayloadDto {
    #[serde(rename = "topicKey")]
    pub topic_key: String,
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl TopicPayloadDto {
    pub fn new(topic_key: String, r#type: Type) -> TopicPayloadDto {
        TopicPayloadDto { topic_key, r#type }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Subscriber")]
    Subscriber,
    #[serde(rename = "Topic")]
    Topic,
}

impl Default for Type {
    fn default() -> Type {
        Self::Subscriber
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopicSubscriberDto {
    #[serde(rename = "_organizationId")]
    pub _organization_id: String,
    #[serde(rename = "_environmentId")]
    pub _environment_id: String,
    #[serde(rename = "_subscriberId")]
    pub _subscriber_id: String,
    #[serde(rename = "_topicId")]
    pub _topic_id: String,
    #[serde(rename = "topicKey")]
    pub topic_key: String,
    #[serde(rename = "externalSubscriberId")]
    pub external_subscriber_id: String,
}

impl TopicSubscriberDto {
    pub fn new(
        _organization_id: String,
        _environment_id: String,
        _subscriber_id: String,
        _topic_id: String,
        topic_key: String,
        external_subscriber_id: String,
    ) -> TopicSubscriberDto {
        TopicSubscriberDto {
            _organization_id,
            _environment_id,
            _subscriber_id,
            _topic_id,
            topic_key,
            external_subscriber_id,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopicDto {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    #[serde(rename = "_organizationId")]
    pub _organization_id: String,
    #[serde(rename = "_environmentId")]
    pub _environment_id: String,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "subscribers")]
    pub subscribers: Vec<String>,
}
