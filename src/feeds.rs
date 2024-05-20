use std::sync::Arc;

use crate::{
    client::{Client, ResponseError},
    models::{CreateFeedRequestDto, FeedResponseDto},
};

pub struct Feeds {
    client: Arc<Client>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteSubscriberResponseDto {
    #[serde(rename = "feedId")]
    feed_id: String,
}

impl Feeds {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        data: CreateFeedRequestDto,
    ) -> Result<FeedResponseDto, ResponseError> {
        self.client.post("/feeds", Some(&data)).await
    }

    pub async fn get(&self) -> Result<Option<Vec<FeedResponseDto>>, ResponseError> {
        self.client.get("/feeds").await
    }

    pub async fn delete(
        &self,
        data: DeleteSubscriberResponseDto,
    ) -> Result<Option<Vec<FeedResponseDto>>, ResponseError> {
        self.client.delete(format!("/feeds/{}", data.feed_id)).await
    }
}
