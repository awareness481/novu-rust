pub mod types;
use crate::{
    client::{Client, ResponseError},
    subscribers::types::RemoveSubscribersRequestDto,
};
use std::{fmt::format, sync::Arc};
pub use types::*;

pub struct Topics {
    client: Arc<Client>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetTopicsDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    page_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<String>,
}

impl Topics {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    pub async fn create(
        self,
        data: CreateTopicRequestDto,
    ) -> Result<CreateTopicRequestDto, ResponseError> {
        self.client.post("/topics", Some(&data)).await
    }

    pub async fn add_subscribers(
        self,
        topic_key: String,
        data: TopicSubscriberDto,
    ) -> Result<(), ResponseError> {
        self.client
            .post(format!("/topics/{topic_key}/subscribers"), Some(&data))
            .await
    }

    pub async fn check_subscriber(
        self,
        topic_key: String,
        external_subscriber_id: String,
    ) -> Result<TopicSubscriberDto, ResponseError> {
        self.client
            .get(format!(
                "/topics/{topic_key}/subscribers/{external_subscriber_id}"
            ))
            .await
    }

    pub async fn remove_subscribers(
        self,
        topic_key: String,
        data: RemoveSubscribersRequestDto,
    ) -> Result<(), ResponseError> {
        self.client
            .post(
                format!("/topics/{topic_key}/subscribers/removal"),
                Some(&data),
            )
            .await
    }

    pub async fn list(&self, data: GetTopicsDto) -> Result<FilterTopicsResponseDto, ResponseError> {
        match qs::to_string(&data) {
            Ok(query) => {
                if !query.is_empty() {
                    self.client.get(format!("/topics?{query}")).await
                } else {
                    self.client.get("/topics").await
                }
            }
            Err(err) => Err(ResponseError::ParseError(err)),
        }
    }

    pub async fn delete(&self, topic_key: String) -> Result<(), ResponseError> {
        self.client.delete(format!("/topics/{topic_key}")).await
    }

    pub async fn get(&self, topic_key: String) -> Result<GetTopicResponseDto, ResponseError> {
        self.client.get(format!("/topics/{topic_key}")).await
    }

    pub async fn rename(
        self,
        topic_key: String,
        data: RenameTopicRequestDto,
    ) -> Result<RenameTopicResponseDto, ResponseError> {
        self.client
            .patch(format!("/topics/{topic_key}"), Some(&data))
            .await
    }
}
