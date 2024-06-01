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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecutionDetailsResponseDto {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    #[serde(rename = "_organizationId")]
    pub _organization_id: String,
    #[serde(rename = "_jobId")]
    pub _job_id: String,
    #[serde(rename = "_environmentId")]
    pub _environment_id: String,
    #[serde(rename = "_notificationId")]
    pub _notification_id: String,
    #[serde(rename = "_notificationTemplateId")]
    pub _notification_template_id: String,
    #[serde(rename = "_subscriberId")]
    pub _subscriber_id: String,
    #[serde(rename = "_messageId", skip_serializing_if = "Option::is_none")]
    pub _message_id: Option<String>,
    #[serde(rename = "providerId", skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    #[serde(rename = "channel")]
    pub channel: Channel,
    #[serde(rename = "detail")]
    pub detail: String,
    #[serde(rename = "source")]
    pub source: Source,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "isTest")]
    pub is_test: bool,
    #[serde(rename = "isRetry")]
    pub is_retry: bool,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
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
    #[serde(rename = "digest")]
    Digest,
    #[serde(rename = "trigger")]
    Trigger,
    #[serde(rename = "delay")]
    Delay,
    #[serde(rename = "custom")]
    Custom,
}

impl Default for Channel {
    fn default() -> Channel {
        Self::InApp
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Source {
    #[serde(rename = "Credentials")]
    Credentials,
    #[serde(rename = "Internal")]
    Internal,
    #[serde(rename = "Payload")]
    Payload,
    #[serde(rename = "Webhook")]
    Webhook,
}

impl Default for Source {
    fn default() -> Source {
        Self::Credentials
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Success")]
    Success,
    #[serde(rename = "Warning")]
    Warning,
    #[serde(rename = "Failed")]
    Failed,
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Queued")]
    Queued,
    #[serde(rename = "ReadConfirmation")]
    ReadConfirmation,
}

impl Default for Status {
    fn default() -> Status {
        Self::Success
    }
}
