use crate::{subscribers::types::SubscriberPayloadDto, tenants::types::TenantPayloadDto};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggerEventResponseDto {
    /// If trigger was acknowledged or not
    #[serde(rename = "acknowledged")]
    pub acknowledged: bool,
    /// Status for trigger
    #[serde(rename = "status")]
    pub status: Status,
    /// In case of an error, this field will contain the error message
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Vec<String>>,
    /// Transaction id for trigger
    #[serde(rename = "transactionId", skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

/// Status for trigger
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "trigger_not_active")]
    TriggerNotActive,
    #[serde(rename = "no_workflow_active_steps_defined")]
    NoWorkflowActiveStepsDefined,
    #[serde(rename = "no_workflow_steps_defined")]
    NoWorkflowStepsDefined,
    #[serde(rename = "processed")]
    Processed,
    #[serde(rename = "subscriber_id_missing")]
    SubscriberIdMissing,
    #[serde(rename = "no_tenant_found")]
    NoTenantFound,
}

impl Default for Status {
    fn default() -> Status {
        Self::Error
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkTriggerEventDto {
    #[serde(rename = "events")]
    pub events: Vec<TriggerEventRequestDto>,
}

impl BulkTriggerEventDto {
    pub fn new(events: Vec<TriggerEventRequestDto>) -> BulkTriggerEventDto {
        BulkTriggerEventDto { events }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggerEventToAllRequestDto {
    /// The trigger identifier associated for the template you wish to send. This identifier can be found on the template page.
    #[serde(rename = "name")]
    pub name: String,
    /// The payload object is used to pass additional custom information that could be used to render the template, or perform routing rules based on it.        This data will also be available when fetching the notifications feed from the API to display certain parts of the UI.
    #[serde(rename = "payload")]
    pub payload: serde_json::Value,
    /// This could be used to override provider specific configurations
    #[serde(rename = "overrides", skip_serializing_if = "Option::is_none")]
    pub overrides: Option<serde_json::Value>,
    /// A unique identifier for this transaction, we will generated a UUID if not provided.
    #[serde(rename = "transactionId", skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(rename = "actor", skip_serializing_if = "Option::is_none")]
    pub actor: Option<Box<TriggerEventRequestDtoActor>>,
    #[serde(rename = "tenant", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Box<TriggerEventRequestDtoTenant>>,
}

/// TriggerEventRequestDtoTenant : It is used to specify a tenant context during trigger event.     If a new tenant object is provided, we will create a new tenant.     
/// It is used to specify a tenant context during trigger event.     If a new tenant object is provided, we will create a new tenant.     
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TriggerEventRequestDtoTenant {
    /// Unique identifier of a tenant in your system
    String(String),
    TenantPayloadDto(Box<TenantPayloadDto>),
}

impl Default for TriggerEventRequestDtoTenant {
    fn default() -> Self {
        Self::String(Default::default())
    }
}

/// TriggerEventRequestDtoActor : It is used to display the Avatar of the provided actor's subscriber id or actor object.     If a new actor object is provided, we will create a new subscriber in our system     
/// It is used to display the Avatar of the provided actor's subscriber id or actor object.     If a new actor object is provided, we will create a new subscriber in our system     
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TriggerEventRequestDtoActor {
    /// Unique identifier of a subscriber in your systems
    String(String),
    SubscriberPayloadDto(Box<SubscriberPayloadDto>),
}

impl Default for TriggerEventRequestDtoActor {
    fn default() -> Self {
        Self::String(Default::default())
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggerEventRequestDto {
    /// The trigger identifier of the workflow you wish to send. This identifier can be found on the workflow page.
    #[serde(rename = "name")]
    pub name: String,
    /// The payload object is used to pass additional custom information that could be used to render the workflow, or perform routing rules based on it.        This data will also be available when fetching the notifications feed from the API to display certain parts of the UI.
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
    /// This could be used to override provider specific configurations
    #[serde(rename = "overrides", skip_serializing_if = "Option::is_none")]
    pub overrides: Option<serde_json::Value>,
    /// The recipients list of people who will receive the notification.
    #[serde(rename = "to")]
    pub to: Vec<Vec<String>>,
    /// A unique identifier for this transaction, we will generated a UUID if not provided.
    #[serde(rename = "transactionId", skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(rename = "actor", skip_serializing_if = "Option::is_none")]
    pub actor: Option<Box<TriggerEventRequestDtoActor>>,
    #[serde(rename = "tenant", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Box<TriggerEventRequestDtoTenant>>,
}
