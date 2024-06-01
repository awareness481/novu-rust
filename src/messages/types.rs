use crate::{
    shared::{EmailBlock, WorkflowResponse},
    subscribers::types::SubscriberResponseDto,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarkAllMessageAsRequestDtoFeedIdentifier {
    String(String),
    Array(Vec<String>),
}

impl Default for MarkAllMessageAsRequestDtoFeedIdentifier {
    fn default() -> Self {
        Self::String(Default::default())
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarkAllMessageAsRequestDto {
    #[serde(rename = "feedIdentifier", skip_serializing_if = "Option::is_none")]
    pub feed_identifier: Option<MarkAllMessageAsRequestDtoFeedIdentifier>,
    /// Mark all subscriber messages as read, unread, seen or unseen
    #[serde(rename = "markAs")]
    pub mark_as: MarkAs,
}

impl MarkAllMessageAsRequestDto {
    pub fn new(
        feed_identifier: Option<MarkAllMessageAsRequestDtoFeedIdentifier>,
        mark_as: MarkAs,
    ) -> MarkAllMessageAsRequestDto {
        MarkAllMessageAsRequestDto {
            feed_identifier,
            mark_as,
        }
    }
}
/// Mark all subscriber messages as read, unread, seen or unseen
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MarkAs {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "seen")]
    Seen,
    #[serde(rename = "unread")]
    Unread,
    #[serde(rename = "unseen")]
    Unseen,
}

impl Default for MarkAs {
    fn default() -> MarkAs {
        Self::Read
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarkMessageActionAsSeenDto {
    /// Message action status
    #[serde(rename = "status")]
    pub status: Status,
    /// Message action payload
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
}
/// Message action status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "done")]
    Done,
}

impl Default for Status {
    fn default() -> Status {
        Self::Pending
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarkMessageAsRequestDtoMessageId {
    String(String),
    Array(Vec<String>),
}

impl Default for MarkMessageAsRequestDtoMessageId {
    fn default() -> Self {
        Self::String(Default::default())
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarkMessageAsRequestDto {
    #[serde(rename = "messageId")]
    pub message_id: Box<MarkMessageAsRequestDtoMessageId>,
    #[serde(rename = "mark")]
    pub mark: Box<MarkMessageFields>,
}

impl MarkMessageAsRequestDto {
    pub fn new(
        message_id: MarkMessageAsRequestDtoMessageId,
        mark: MarkMessageFields,
    ) -> MarkMessageAsRequestDto {
        MarkMessageAsRequestDto {
            message_id: Box::new(message_id),
            mark: Box::new(mark),
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarkMessageFields {
    #[serde(rename = "seen", skip_serializing_if = "Option::is_none")]
    pub seen: Option<bool>,
    #[serde(rename = "read", skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageActionResult {
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<MessageActionType>,
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MessageActionType {
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "secondary")]
    Secondary,
    #[serde(rename = "clicked")]
    Clicked,
}

impl Default for MessageActionType {
    fn default() -> MessageActionType {
        Self::Primary
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageAction {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<MessageActionStatus>,
    #[serde(rename = "buttons", skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<MessageButton>>,
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<MessageActionResult>>,
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MessageActionStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "done")]
    Done,
}

impl Default for MessageActionStatus {
    fn default() -> MessageActionStatus {
        Self::Pending
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageButton {
    #[serde(rename = "type")]
    pub r#type: MessageButtonType,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "resultContent", skip_serializing_if = "Option::is_none")]
    pub result_content: Option<String>,
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MessageButtonType {
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "secondary")]
    Secondary,
    #[serde(rename = "clicked")]
    Clicked,
}

impl Default for MessageButtonType {
    fn default() -> MessageButtonType {
        Self::Primary
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageCtaData {
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageCta {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<MessageCtaType>,
    #[serde(rename = "data")]
    pub data: Box<MessageCtaData>,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Box<MessageAction>>,
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MessageCtaType {
    #[serde(rename = "redirect")]
    Redirect,
}

impl Default for MessageCtaType {
    fn default() -> MessageCtaType {
        Self::Redirect
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageMarkAsRequestDto {
    #[serde(rename = "messageId")]
    pub message_id: Box<MarkMessageAsRequestDtoMessageId>,
    #[serde(rename = "markAs")]
    pub mark_as: MarkAs,
}

impl MessageMarkAsRequestDto {
    pub fn new(
        message_id: MarkMessageAsRequestDtoMessageId,
        mark_as: MarkAs,
    ) -> MessageMarkAsRequestDto {
        MessageMarkAsRequestDto {
            message_id: Box::new(message_id),
            mark_as,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageResponseDtoContent {
    EmailBlock(Box<EmailBlock>),
    String(String),
}

impl Default for MessageResponseDtoContent {
    fn default() -> Self {
        Self::EmailBlock(Default::default())
    }
}
///

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageResponseDto {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    #[serde(rename = "_templateId")]
    pub _template_id: String,
    #[serde(rename = "_environmentId")]
    pub _environment_id: String,
    #[serde(rename = "_messageTemplateId")]
    pub _message_template_id: String,
    #[serde(rename = "_organizationId")]
    pub _organization_id: String,
    #[serde(rename = "_notificationId")]
    pub _notification_id: String,
    #[serde(rename = "_subscriberId")]
    pub _subscriber_id: String,
    #[serde(rename = "subscriber", skip_serializing_if = "Option::is_none")]
    pub subscriber: Option<Box<SubscriberResponseDto>>,
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<Box<WorkflowResponse>>,
    #[serde(rename = "templateIdentifier", skip_serializing_if = "Option::is_none")]
    pub template_identifier: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "content")]
    pub content: Box<MessageResponseDtoContent>,
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "channel")]
    pub channel: Channel,
    #[serde(rename = "seen")]
    pub seen: bool,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "directWebhookUrl", skip_serializing_if = "Option::is_none")]
    pub direct_webhook_url: Option<String>,
    #[serde(rename = "providerId", skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,
    #[serde(rename = "deviceTokens", skip_serializing_if = "Option::is_none")]
    pub device_tokens: Option<Vec<String>>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "lastSeenDate")]
    pub last_seen_date: String,
    #[serde(rename = "cta")]
    pub cta: Box<MessageCta>,
    #[serde(rename = "_feedId", skip_serializing_if = "Option::is_none")]
    pub _feed_id: Option<String>,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "errorId")]
    pub error_id: String,
    #[serde(rename = "errorText")]
    pub error_text: String,
    /// The payload that was used to send the notification trigger
    #[serde(rename = "payload")]
    pub payload: serde_json::Value,
    /// Provider specific overrides used when triggering the notification
    #[serde(rename = "overrides")]
    pub overrides: serde_json::Value,
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Channel {
    #[serde(rename = "in_app")]
    InApp,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "chat")]
    Chat,
    #[serde(rename = "push")]
    Push,
}

impl Default for Channel {
    fn default() -> Channel {
        Self::InApp
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteMessageResponseDto {
    /// A boolean stating the success of the action
    #[serde(rename = "acknowledged")]
    pub acknowledged: bool,
    /// The status enum for the performed action
    #[serde(rename = "status")]
    pub status: DeletedStatus,
}

impl DeleteMessageResponseDto {
    pub fn new(acknowledged: bool, status: DeletedStatus) -> DeleteMessageResponseDto {
        DeleteMessageResponseDto {
            acknowledged,
            status,
        }
    }
}
/// The status enum for the performed action
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DeletedStatus {
    #[serde(rename = "deleted")]
    Deleted,
}

impl Default for DeletedStatus {
    fn default() -> DeletedStatus {
        Self::Deleted
    }
}
