use crate::{
    client::{Client, Response, ResponseError},
    feeds::DeleteSubscriberResponseDto,
    models::{
        message_mark_as_request_dto::MarkAs, subscriber_payload_dto::SubscriberPayloadWithIdDto,
        BulkSubscriberCreateDto, ChannelCredentials, GetSubscriberPreferencesResponseDto,
        MarkAllMessageAsRequestDto, MarkAllMessageAsRequestDtoFeedIdentifier,
        MarkMessageActionAsSeenDto, MarkMessageAsRequestDtoMessageId, MarkMessageFields,
        MessageMarkAsRequestDto, MessageResponseDto, PaginatedResponseDto, SubscriberPayloadDto,
        SubscriberResponseDto, UnseenCountResponse, UpdateSubscriberChannelRequestDto,
        UpdateSubscriberGlobalPreferencesRequestDto, UpdateSubscriberOnlineFlagRequestDto,
        UpdateSubscriberPreferenceRequestDto, UpdateSubscriberPreferenceResponseDto,
    },
};
use std::str::FromStr;
use std::sync::Arc;
use strum::Display;
use strum_macros::EnumString;

pub struct Subscribers {
    client: Arc<Client>,
}

#[derive(Debug, PartialEq, EnumString, Serialize, Deserialize, Display)]
#[strum(serialize_all = "lowercase")]
pub enum PreferenceLevel {
    GLOBAL,
    TEMPLATE,
}

#[derive(Debug, PartialEq, EnumString, Serialize, Deserialize, Display)]
#[strum(serialize_all = "lowercase")]
pub enum ButtonType {
    PRIMARY,
    SECONDARY,
    CLICKED,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetSubscribersDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetNotificationsFeedDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seen: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetUnseenCountDto {
    seen: bool,
    limit: u32,
}

impl Subscribers {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    pub async fn list(
        self,
        data: GetSubscribersDto,
    ) -> Result<PaginatedResponseDto<SubscriberResponseDto>, ResponseError> {
        match qs::to_string(&data) {
            Ok(query) => {
                if !query.is_empty() {
                    self.client.get(format!("/subscibers?{query}")).await
                } else {
                    self.client.get("/subscibers").await
                }
            }
            Err(err) => Err(ResponseError::ParseError(err)),
        }
    }

    pub async fn get(self, subscriber_id: String) -> Result<SubscriberResponseDto, ResponseError> {
        self.client
            .get(format!("/subscribers/${subscriber_id}"))
            .await
    }

    pub async fn identify(
        self,
        data: SubscriberPayloadWithIdDto,
    ) -> Result<SubscriberResponseDto, ResponseError> {
        self.client.post("/subscribers", Some(&data)).await
    }

    pub async fn bulk_create(self, data: BulkSubscriberCreateDto) -> Result<(), ResponseError> {
        self.client.post("/subscribers/bulk", Some(&data)).await
    }

    pub async fn update(
        self,
        subscriber_id: String,
        data: SubscriberPayloadDto,
    ) -> Result<SubscriberResponseDto, ResponseError> {
        self.client
            .put(format!("/subscribers/${subscriber_id}"), &data)
            .await
    }

    pub async fn set_credentials(
        self,
        subscriber_id: String,
        data: UpdateSubscriberChannelRequestDto,
    ) -> Result<SubscriberResponseDto, ResponseError> {
        self.client
            .put(format!("/subscribers/${subscriber_id}/credentials"), &data)
            .await
    }

    pub async fn delete_credentials(
        self,
        subscriber_id: String,
        provider_id: String,
    ) -> Result<(), ResponseError> {
        self.client
            .delete(format!(
                "/subscribers/${subscriber_id}/credentials/${provider_id}"
            ))
            .await
    }

    pub async fn update_online_status(
        self,
        subscriber_id: String,
        data: UpdateSubscriberOnlineFlagRequestDto,
    ) -> Result<SubscriberResponseDto, ResponseError> {
        self.client
            .patch(
                format!("/subscribers/{subscriber_id}/online-status"),
                Some(&data),
            )
            .await
    }

    pub async fn delete(
        self,
        subscriber_id: String,
    ) -> Result<DeleteSubscriberResponseDto, ResponseError> {
        self.client
            .delete(format!("/subscribers/{subscriber_id}"))
            .await
    }

    pub async fn get_preference(
        self,
        subscriber_id: String,
    ) -> Result<UpdateSubscriberPreferenceResponseDto, ResponseError> {
        self.client
            .get(format!("/subscribers/${subscriber_id}/preferences"))
            .await
    }

    pub async fn get_global_preference(
        self,
        subscriber_id: String,
    ) -> Result<GetSubscriberPreferencesResponseDto, ResponseError> {
        self.client
            .get(format!(
                "/subscribers/${subscriber_id}/preferences/${}",
                PreferenceLevel::GLOBAL
            ))
            .await
    }

    pub async fn get_preference_by_level(
        self,
        subscriber_id: String,
        level: PreferenceLevel,
    ) -> Result<GetSubscriberPreferencesResponseDto, ResponseError> {
        self.client
            .get(format!(
                "/subscribers/${subscriber_id}/preferences/${level}"
            ))
            .await
    }

    pub async fn update_by_preference(
        self,
        subscriber_id: String,
        template_id: String,
        data: UpdateSubscriberPreferenceRequestDto,
    ) -> Result<UpdateSubscriberPreferenceResponseDto, ResponseError> {
        self.client
            .patch(
                format!("/subscribers/${subscriber_id}/preferences/${template_id}"),
                Some(&data),
            )
            .await
    }

    pub async fn update_global_preference(
        self,
        subscriber_id: String,
        data: UpdateSubscriberGlobalPreferencesRequestDto,
    ) -> Result<UpdateSubscriberPreferenceResponseDto, ResponseError> {
        self.client
            .patch(
                format!("/subscribers/${subscriber_id}/preferences"),
                Some(&data),
            )
            .await
    }

    pub async fn get_notifications_feed(
        self,
        subscriber_id: String,
        data: GetNotificationsFeedDto,
    ) -> Result<PaginatedResponseDto<MessageResponseDto>, ResponseError> {
        match qs::to_string(&data) {
            Ok(query) => {
                if !query.is_empty() {
                    self.client
                        .get(format!(
                            "/subscribers/{subscriber_id}/notifications/feed?{query}"
                        ))
                        .await
                } else {
                    self.client
                        .get(format!("/subscribers/{subscriber_id}/notifications/feed"))
                        .await
                }
            }
            Err(err) => Err(ResponseError::ParseError(err)),
        }
    }

    pub async fn get_unseen_count(
        self,
        subscriber_id: String,
        data: GetUnseenCountDto,
    ) -> Result<UnseenCountResponse, ResponseError> {
        match qs::to_string(&data) {
            Ok(query) => {
                self.client
                    .get(format!(
                        "/subscribers/{subscriber_id}/notifications/unseen?{query}"
                    ))
                    .await
            }
            Err(err) => Err(ResponseError::ParseError(err)),
        }
    }

    pub async fn mark_message_seen(
        self,
        subscriber_id: String,
        message_id: String,
    ) -> Result<serde_json::Value, ResponseError> {
        let data = MessageMarkAsRequestDto::new(
            MarkMessageAsRequestDtoMessageId::String(message_id),
            MarkAs::Seen,
        );

        self.client
            .post(
                format!("/subscribers/${subscriber_id}/messages/mark-as"),
                Some(&data),
            )
            .await
    }

    pub async fn mark_message_read(
        self,
        subscriber_id: String,
        message_id: String,
    ) -> Result<serde_json::Value, ResponseError> {
        let data = MessageMarkAsRequestDto::new(
            MarkMessageAsRequestDtoMessageId::String(message_id),
            MarkAs::Read,
        );

        self.client
            .post(
                format!("/subscribers/${subscriber_id}/messages/mark-as"),
                Some(&data),
            )
            .await
    }

    pub async fn mark_message_as(
        self,
        subscriber_id: String,
        message_id: String,
        mark_as: MarkAs,
    ) -> Result<serde_json::Value, ResponseError> {
        let data = MessageMarkAsRequestDto::new(
            MarkMessageAsRequestDtoMessageId::String(message_id),
            mark_as,
        );

        self.client
            .post(
                format!("/subscribers/${subscriber_id}/messages/mark-as"),
                Some(&data),
            )
            .await
    }

    pub async fn mark_all_message_as(
        self,
        subscriber_id: String,
        data: MarkAllMessageAsRequestDto,
    ) -> Result<u32, ResponseError> {
        self.client
            .post(
                format!("/v1/subscribers/{subscriber_id}/messages/mark-all"),
                Some(&data),
            )
            .await
    }

    pub async fn mark_message_action_seen(
        self,
        subscriber_id: String,
        message_id: String,
        button_type: ButtonType,
        data: MarkMessageActionAsSeenDto,
    ) -> Result<MessageResponseDto, ResponseError> {
        self.client
            .post(
                format!(
                    "/subscribers/${subscriber_id}/messages/${message_id}/actions/${button_type}"
                ),
                Some(&data),
            )
            .await
    }
}
