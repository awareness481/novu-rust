pub mod types;
use crate::client::{Client, ResponseError};
use crate::shared::activities::ActivitiesResponseDto;
use std::str::FromStr;
use std::sync::Arc;
use strum_macros::EnumString;

pub use types::*;

pub struct Messages {
    client: Arc<Client>,
}

#[derive(Clone, Serialize, Deserialize, EnumString, Debug, PartialEq)]
pub enum ChannelTypeEnum {
    #[strum(serialize = "in_app")]
    InApp,
    #[strum(serialize = "email")]
    Email,
    #[strum(serialize = "sms")]
    Sms,
    #[strum(serialize = "chat")]
    Chat,
    #[strum(serialize = "push")]
    Push,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetMessagesDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    channel: Option<ChannelTypeEnum>,
    #[serde(rename = "subsciberId", skip_serializing_if = "Option::is_none")]
    subscriber_id: Option<String>,
    #[serde(rename = "transactionId", skip_serializing_if = "Option::is_none")]
    transaction_id: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl Messages {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    pub async fn list(&self, data: GetMessagesDto) -> Result<ActivitiesResponseDto, ResponseError> {
        match qs::to_string(&data) {
            Ok(query) => {
                if !query.is_empty() {
                    self.client.get(format!("/messages/?{query}")).await
                } else {
                    self.client.get("/messages").await
                }
            }
            Err(err) => Err(ResponseError::ParseError(err)),
        }
    }

    pub async fn delete_by_id(
        self,
        message_id: String,
    ) -> Result<DeleteMessageResponseDto, ResponseError> {
        self.client.delete(format!("/messages/{message_id}")).await
    }
}
