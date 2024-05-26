use super::{NotificationTrigger, StepFilter};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivitiesResponseDto {
    #[serde(rename = "hasMore")]
    pub has_more: bool,
    #[serde(rename = "data")]
    pub data: Vec<ActivityNotificationResponseDto>,
    #[serde(rename = "pageSize")]
    pub page_size: f64,
    #[serde(rename = "page")]
    pub page: f64,
}

impl ActivitiesResponseDto {
    pub fn new(
        has_more: bool,
        data: Vec<ActivityNotificationResponseDto>,
        page_size: f64,
        page: f64,
    ) -> ActivitiesResponseDto {
        ActivitiesResponseDto {
            has_more,
            data,
            page_size,
            page,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityGraphStatesResponse {
    #[serde(rename = "_id")]
    pub _id: String,
    #[serde(rename = "count")]
    pub count: f64,
    #[serde(rename = "templates")]
    pub templates: Vec<String>,
    #[serde(rename = "channels")]
    pub channels: Vec<Channels>,
}

impl ActivityGraphStatesResponse {
    pub fn new(
        _id: String,
        count: f64,
        templates: Vec<String>,
        channels: Vec<Channels>,
    ) -> ActivityGraphStatesResponse {
        ActivityGraphStatesResponse {
            _id,
            count,
            templates,
            channels,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Channels {
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

impl Default for Channels {
    fn default() -> Channels {
        Self::InApp
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityNotificationExecutionDetailResponseDto {
    #[serde(rename = "_id")]
    pub _id: String,
    #[serde(rename = "_jobId")]
    pub _job_id: String,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "detail")]
    pub detail: String,
    #[serde(rename = "isRetry")]
    pub is_retry: bool,
    #[serde(rename = "isTest")]
    pub is_test: bool,
    #[serde(rename = "providerId")]
    pub provider_id: serde_json::Value,
    #[serde(rename = "raw", skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
    #[serde(rename = "source")]
    pub source: Source,
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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityNotificationJobResponseDto {
    #[serde(rename = "_id")]
    pub _id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "digest", skip_serializing_if = "Option::is_none")]
    pub digest: Option<serde_json::Value>,
    #[serde(rename = "executionDetails")]
    pub execution_details: Vec<ActivityNotificationExecutionDetailResponseDto>,
    #[serde(rename = "step")]
    pub step: Box<ActivityNotificationStepResponseDto>,
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
    #[serde(rename = "providerId")]
    pub provider_id: serde_json::Value,
    #[serde(rename = "status")]
    pub status: String,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityNotificationResponseDto {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    #[serde(rename = "_environmentId")]
    pub _environment_id: String,
    #[serde(rename = "_organizationId")]
    pub _organization_id: String,
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "channels", skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<Channels>>,
    #[serde(rename = "subscriber", skip_serializing_if = "Option::is_none")]
    pub subscriber: Option<Box<ActivityNotificationSubscriberResponseDto>>,
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<Box<ActivityNotificationTemplateResponseDto>>,
    #[serde(rename = "jobs", skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<ActivityNotificationJobResponseDto>>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityNotificationStepResponseDto {
    #[serde(rename = "_id")]
    pub _id: String,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "filters")]
    pub filters: Box<StepFilter>,
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<serde_json::Value>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityNotificationSubscriberResponseDto {
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "_id")]
    pub _id: String,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityNotificationTemplateResponseDto {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "triggers")]
    pub triggers: Vec<NotificationTrigger>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityStatsResponseDto {
    #[serde(rename = "weeklySent")]
    pub weekly_sent: f64,
    #[serde(rename = "monthlySent")]
    pub monthly_sent: f64,
}

impl ActivityStatsResponseDto {
    pub fn new(weekly_sent: f64, monthly_sent: f64) -> ActivityStatsResponseDto {
        ActivityStatsResponseDto {
            weekly_sent,
            monthly_sent,
        }
    }
}
