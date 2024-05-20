use crate::{
    client::{Client, ResponseError},
    models::ExecutionDetailsResponseDto,
};
use std::sync::Arc;

pub struct ExecutationDetails {
    client: Arc<Client>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecutionDetailsRequestDto {
    #[serde(rename = "notificationId")]
    notification_id: String,
    #[serde(rename = "subsciberId")]
    subscriber_id: String,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetExecutionDetailsDto {
    #[serde(rename = "notificationId")]
    notification_id: String,
    #[serde(rename = "subscriberId")]
    subscriber_id: String,
}

impl ExecutationDetails {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    pub async fn get(
        &self,
        data: ExecutionDetailsRequestDto,
    ) -> Result<ExecutionDetailsResponseDto, ResponseError> {
        match qs::to_string(&data) {
            Ok(query) => self.client.get(format!("/execution-details?{query}")).await,
            Err(e) => Err(ResponseError::ParseError(e)),
        }
    }
}
