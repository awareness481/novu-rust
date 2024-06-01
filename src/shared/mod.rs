use crate::notification_groups::types::NotificationGroup;

pub mod activities;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataNumberDto {
    #[serde(rename = "data")]
    pub data: f64,
}

impl DataNumberDto {
    pub fn new(data: f64) -> DataNumberDto {
        DataNumberDto { data }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TemplateResponse {
    /// Unique identifier of the workflow
    #[serde(rename = "_id")]
    pub _id: String,
    /// Name of the workflow
    #[serde(rename = "name")]
    pub name: String,
    /// Critical templates will always be delivered to the end user and should be hidden from the subscriber preferences screen
    #[serde(rename = "critical")]
    pub critical: bool,
    /// Triggers are the events that will trigger the workflow.
    #[serde(rename = "triggers")]
    pub triggers: Vec<String>,
}

impl TemplateResponse {
    pub fn new(
        _id: String,
        name: String,
        critical: bool,
        triggers: Vec<String>,
    ) -> TemplateResponse {
        TemplateResponse {
            _id,
            name,
            critical,
            triggers,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Preference {
    /// Sets if the workflow is fully enabled for all channels or not for the subscriber.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Subscriber preferences for the different channels regarding this workflow
    #[serde(rename = "channels")]
    pub channels: Box<PreferenceChannels>,
}

impl Preference {
    pub fn new(enabled: bool, channels: PreferenceChannels) -> Preference {
        Preference {
            enabled,
            channels: Box::new(channels),
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PreferenceChannels {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<bool>,
    #[serde(rename = "sms", skip_serializing_if = "Option::is_none")]
    pub sms: Option<bool>,
    #[serde(rename = "in_app", skip_serializing_if = "Option::is_none")]
    pub in_app: Option<bool>,
    #[serde(rename = "chat", skip_serializing_if = "Option::is_none")]
    pub chat: Option<bool>,
    #[serde(rename = "push", skip_serializing_if = "Option::is_none")]
    pub push: Option<bool>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelCredentials {
    /// Webhook url used by chat app integrations. The webhook should be obtained from the chat app provider.
    #[serde(rename = "webhookUrl")]
    pub webhook_url: String,
    /// Channel specification for Mattermost chat notifications
    #[serde(rename = "channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    /// Contains an array of the subscriber device tokens for a given provider. Used on Push integrations
    #[serde(rename = "deviceTokens", skip_serializing_if = "Option::is_none")]
    pub device_tokens: Option<Vec<String>>,
    /// alert_uid for grafana on-call webhook payload
    #[serde(rename = "alertUid", skip_serializing_if = "Option::is_none")]
    pub alert_uid: Option<String>,
    /// title to be used with grafana on call webhook
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// image_url property fo grafana on call webhook
    #[serde(rename = "imageUrl", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// state property fo grafana on call webhook
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// link_to_upstream_details property fo grafana on call webhook
    #[serde(rename = "externalUrl", skip_serializing_if = "Option::is_none")]
    pub external_url: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelPreference {
    /// The type of channel that is enabled or not
    #[serde(rename = "type")]
    pub r#type: Type,
    /// If channel is enabled or not
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

impl ChannelPreference {
    pub fn new(r#type: Type, enabled: bool) -> ChannelPreference {
        ChannelPreference { r#type, enabled }
    }
}
/// The type of channel that is enabled or not
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
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

impl Default for Type {
    fn default() -> Type {
        Self::InApp
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginatedResponseDto<T> {
    /// The current page of the paginated response
    #[serde(rename = "page")]
    pub page: f64,
    /// Does the list have more items to fetch
    #[serde(rename = "hasMore")]
    pub has_more: bool,
    /// Number of items on each page
    #[serde(rename = "pageSize")]
    pub page_size: f64,
    /// The list of items matching the query
    #[serde(rename = "data")]
    pub data: Vec<T>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelSettings {
    /// The provider identifier for the credentials
    #[serde(rename = "providerId")]
    pub provider_id: ProviderId,
    /// The integration identifier
    #[serde(
        rename = "integrationIdentifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub integration_identifier: Option<String>,
    /// Credentials payload for the specified provider
    #[serde(rename = "credentials")]
    pub credentials: Box<ChannelCredentials>,
    /// Id of the integration that is used for this channel
    #[serde(rename = "_integrationId")]
    pub _integration_id: String,
}

impl ChannelSettings {
    pub fn new(
        provider_id: ProviderId,
        credentials: ChannelCredentials,
        _integration_id: String,
    ) -> ChannelSettings {
        ChannelSettings {
            provider_id,
            integration_identifier: None,
            credentials: Box::new(credentials),
            _integration_id,
        }
    }
}
/// The provider identifier for the credentials
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProviderId {
    #[serde(rename = "slack")]
    Slack,
    #[serde(rename = "discord")]
    Discord,
    #[serde(rename = "msteams")]
    Msteams,
    #[serde(rename = "mattermost")]
    Mattermost,
    #[serde(rename = "ryver")]
    Ryver,
    #[serde(rename = "zulip")]
    Zulip,
    #[serde(rename = "grafana-on-call")]
    GrafanaOnCall,
    #[serde(rename = "getstream")]
    Getstream,
    #[serde(rename = "rocket-chat")]
    RocketChat,
    #[serde(rename = "whatsapp-business")]
    WhatsappBusiness,
    #[serde(rename = "fcm")]
    Fcm,
    #[serde(rename = "apns")]
    Apns,
    #[serde(rename = "expo")]
    Expo,
    #[serde(rename = "one-signal")]
    OneSignal,
    #[serde(rename = "pushpad")]
    Pushpad,
    #[serde(rename = "push-webhook")]
    PushWebhook,
    #[serde(rename = "pusher-beams")]
    PusherBeams,
}

impl Default for ProviderId {
    fn default() -> ProviderId {
        Self::Slack
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationTriggerVariable {
    #[serde(rename = "name")]
    pub name: String,
}

impl NotificationTriggerVariable {
    pub fn new(name: String) -> NotificationTriggerVariable {
        NotificationTriggerVariable { name }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StepFilter {
    #[serde(rename = "isNegated")]
    pub is_negated: bool,
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "value")]
    pub value: Value,
    #[serde(rename = "children")]
    pub children: Vec<FieldFilterPart>,
}

impl StepFilter {
    pub fn new(
        is_negated: bool,
        r#type: Type,
        value: Value,
        children: Vec<FieldFilterPart>,
    ) -> StepFilter {
        StepFilter {
            is_negated,
            r#type,
            value,
            children,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FilterType {
    #[serde(rename = "BOOLEAN")]
    Boolean,
    #[serde(rename = "TEXT")]
    Text,
    #[serde(rename = "DATE")]
    Date,
    #[serde(rename = "NUMBER")]
    Number,
    #[serde(rename = "STATEMENT")]
    Statement,
    #[serde(rename = "LIST")]
    List,
    #[serde(rename = "MULTI_LIST")]
    MultiList,
    #[serde(rename = "GROUP")]
    Group,
}

impl Default for FilterType {
    fn default() -> FilterType {
        Self::Boolean
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "AND")]
    And,
    #[serde(rename = "OR")]
    Or,
}

impl Default for Value {
    fn default() -> Value {
        Self::And
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldFilterPart {
    #[serde(rename = "field")]
    pub field: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "operator")]
    pub operator: Operator,
    #[serde(rename = "on")]
    pub on: On,
}

impl FieldFilterPart {
    pub fn new(field: String, value: String, operator: Operator, on: On) -> FieldFilterPart {
        FieldFilterPart {
            field,
            value,
            operator,
            on,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "LARGER")]
    Larger,
    #[serde(rename = "SMALLER")]
    Smaller,
    #[serde(rename = "LARGER_EQUAL")]
    LargerEqual,
    #[serde(rename = "SMALLER_EQUAL")]
    SmallerEqual,
    #[serde(rename = "EQUAL")]
    Equal,
    #[serde(rename = "NOT_EQUAL")]
    NotEqual,
    #[serde(rename = "ALL_IN")]
    AllIn,
    #[serde(rename = "ANY_IN")]
    AnyIn,
    #[serde(rename = "NOT_IN")]
    NotIn,
    #[serde(rename = "BETWEEN")]
    Between,
    #[serde(rename = "NOT_BETWEEN")]
    NotBetween,
    #[serde(rename = "LIKE")]
    Like,
    #[serde(rename = "NOT_LIKE")]
    NotLike,
    #[serde(rename = "IN")]
    In,
}

impl Default for Operator {
    fn default() -> Operator {
        Self::Larger
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum On {
    #[serde(rename = "subscriber")]
    Subscriber,
    #[serde(rename = "payload")]
    Payload,
}

impl Default for On {
    fn default() -> On {
        Self::Subscriber
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationTrigger {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "variables")]
    pub variables: Vec<NotificationTriggerVariable>,
    #[serde(
        rename = "subscriberVariables",
        skip_serializing_if = "Option::is_none"
    )]
    pub subscriber_variables: Option<Vec<NotificationTriggerVariable>>,
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NotificationTriggerType {
    #[serde(rename = "event")]
    Event,
}

impl Default for NotificationTriggerType {
    fn default() -> NotificationTriggerType {
        Self::Event
    }
}
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailBlock {
    #[serde(rename = "type")]
    pub r#type: EmailType,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "styles", skip_serializing_if = "Option::is_none")]
    pub styles: Option<Box<EmailBlockStyles>>,
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EmailType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "button")]
    Button,
}

impl Default for EmailType {
    fn default() -> EmailType {
        Self::Text
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailBlockStyles {
    #[serde(rename = "textAlign", skip_serializing_if = "Option::is_none")]
    pub text_align: Option<TextAlign>,
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TextAlign {
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "center")]
    Center,
}

impl Default for TextAlign {
    fn default() -> TextAlign {
        Self::Left
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowResponse {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "draft")]
    pub draft: bool,
    #[serde(rename = "preferenceSettings")]
    pub preference_settings: Box<PreferenceChannels>,
    #[serde(rename = "critical")]
    pub critical: bool,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "steps")]
    pub steps: Vec<NotificationStep>,
    #[serde(rename = "_organizationId")]
    pub _organization_id: String,
    #[serde(rename = "_creatorId")]
    pub _creator_id: String,
    #[serde(rename = "_environmentId")]
    pub _environment_id: String,
    #[serde(rename = "triggers")]
    pub triggers: Vec<NotificationTrigger>,
    #[serde(rename = "_notificationGroupId")]
    pub _notification_group_id: String,
    #[serde(rename = "_parentId", skip_serializing_if = "Option::is_none")]
    pub _parent_id: Option<String>,
    #[serde(rename = "deleted")]
    pub deleted: bool,
    #[serde(rename = "deletedAt")]
    pub deleted_at: String,
    #[serde(rename = "deletedBy")]
    pub deleted_by: String,
    #[serde(rename = "notificationGroup", skip_serializing_if = "Option::is_none")]
    pub notification_group: Option<Box<NotificationGroup>>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(
        rename = "workflowIntegrationStatus",
        skip_serializing_if = "Option::is_none"
    )]
    pub workflow_integration_status: Option<serde_json::Value>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationStep {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "_templateId", skip_serializing_if = "Option::is_none")]
    pub _template_id: Option<String>,
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "shouldStopOnFail", skip_serializing_if = "Option::is_none")]
    pub should_stop_on_fail: Option<bool>,
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<serde_json::Value>,
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<StepFilter>>,
    #[serde(rename = "_parentId", skip_serializing_if = "Option::is_none")]
    pub _parent_id: Option<serde_json::Value>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<NotificationStepVariantMetadata>>,
    #[serde(rename = "replyCallback", skip_serializing_if = "Option::is_none")]
    pub reply_callback: Option<serde_json::Value>,
    #[serde(rename = "variants", skip_serializing_if = "Option::is_none")]
    pub variants: Option<Box<NotificationStepVariant>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NotificationStepVariantMetadata {
    DigestRegularMetadata(Box<DigestRegularMetadata>),
    DigestTimedMetadata(Box<DigestTimedMetadata>),
    DelayRegularMetadata(Box<DelayRegularMetadata>),
    DelayScheduledMetadata(Box<DelayScheduledMetadata>),
}

impl Default for NotificationStepVariantMetadata {
    fn default() -> Self {
        Self::DigestRegularMetadata(Default::default())
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationStepVariant {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "_templateId", skip_serializing_if = "Option::is_none")]
    pub _template_id: Option<String>,
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "shouldStopOnFail", skip_serializing_if = "Option::is_none")]
    pub should_stop_on_fail: Option<bool>,
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<serde_json::Value>,
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<StepFilter>>,
    #[serde(rename = "_parentId", skip_serializing_if = "Option::is_none")]
    pub _parent_id: Option<serde_json::Value>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<NotificationStepVariantMetadata>>,
    #[serde(rename = "replyCallback", skip_serializing_if = "Option::is_none")]
    pub reply_callback: Option<serde_json::Value>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DigestRegularMetadata {
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
    #[serde(rename = "digestKey", skip_serializing_if = "Option::is_none")]
    pub digest_key: Option<String>,
    #[serde(rename = "type")]
    pub r#type: DigestRegularType,
    #[serde(rename = "backoff", skip_serializing_if = "Option::is_none")]
    pub backoff: Option<bool>,
    #[serde(rename = "backoffAmount", skip_serializing_if = "Option::is_none")]
    pub backoff_amount: Option<f64>,
    #[serde(rename = "backoffUnit", skip_serializing_if = "Option::is_none")]
    pub backoff_unit: Option<BackoffUnit>,
    #[serde(rename = "updateMode", skip_serializing_if = "Option::is_none")]
    pub update_mode: Option<bool>,
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Unit {
    #[serde(rename = "seconds")]
    Seconds,
    #[serde(rename = "minutes")]
    Minutes,
    #[serde(rename = "hours")]
    Hours,
    #[serde(rename = "days")]
    Days,
    #[serde(rename = "weeks")]
    Weeks,
    #[serde(rename = "months")]
    Months,
}

impl Default for Unit {
    fn default() -> Unit {
        Self::Seconds
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DigestRegularType {
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "backoff")]
    Backoff,
}

impl Default for DigestRegularType {
    fn default() -> DigestRegularType {
        Self::Regular
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BackoffUnit {
    #[serde(rename = "seconds")]
    Seconds,
    #[serde(rename = "minutes")]
    Minutes,
    #[serde(rename = "hours")]
    Hours,
    #[serde(rename = "days")]
    Days,
    #[serde(rename = "weeks")]
    Weeks,
    #[serde(rename = "months")]
    Months,
}

impl Default for BackoffUnit {
    fn default() -> BackoffUnit {
        Self::Seconds
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DigestTimedMetadata {
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
    #[serde(rename = "digestKey", skip_serializing_if = "Option::is_none")]
    pub digest_key: Option<String>,
    #[serde(rename = "type")]
    pub r#type: DigestTimedType,
    #[serde(rename = "timed", skip_serializing_if = "Option::is_none")]
    pub timed: Option<Box<TimedConfig>>,
}
///

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DigestTimedType {
    #[serde(rename = "timed")]
    Timed,
}

impl Default for DigestTimedType {
    fn default() -> DigestTimedType {
        Self::Timed
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DelayRegularMetadata {
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
    #[serde(rename = "type")]
    pub r#type: DelayRegulaType,
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DelayRegulaType {
    #[serde(rename = "regular")]
    Regular,
}

impl Default for DelayRegulaType {
    fn default() -> DelayRegulaType {
        Self::Regular
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DelayScheduledMetadata {
    #[serde(rename = "type")]
    pub r#type: DelayScheduledType,
    #[serde(rename = "delayPath")]
    pub delay_path: String,
}

impl DelayScheduledMetadata {
    pub fn new(r#type: DelayScheduledType, delay_path: String) -> DelayScheduledMetadata {
        DelayScheduledMetadata { r#type, delay_path }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DelayScheduledType {
    #[serde(rename = "scheduled")]
    Scheduled,
}

impl Default for DelayScheduledType {
    fn default() -> DelayScheduledType {
        Self::Scheduled
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimedConfig {
    #[serde(rename = "atTime", skip_serializing_if = "Option::is_none")]
    pub at_time: Option<String>,
    #[serde(rename = "weekDays", skip_serializing_if = "Option::is_none")]
    pub week_days: Option<Vec<WeekDays>>,
    #[serde(rename = "monthDays", skip_serializing_if = "Option::is_none")]
    pub month_days: Option<Vec<String>>,
    #[serde(rename = "ordinal", skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<Ordinal>,
    #[serde(rename = "ordinalValue", skip_serializing_if = "Option::is_none")]
    pub ordinal_value: Option<OrdinalValue>,
    #[serde(rename = "monthlyType", skip_serializing_if = "Option::is_none")]
    pub monthly_type: Option<MonthlyType>,
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WeekDays {
    #[serde(rename = "monday")]
    Monday,
    #[serde(rename = "tuesday")]
    Tuesday,
    #[serde(rename = "wednesday")]
    Wednesday,
    #[serde(rename = "thursday")]
    Thursday,
    #[serde(rename = "friday")]
    Friday,
    #[serde(rename = "saturday")]
    Saturday,
    #[serde(rename = "sunday")]
    Sunday,
}

impl Default for WeekDays {
    fn default() -> WeekDays {
        Self::Monday
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Ordinal {
    #[serde(rename = "1")]
    Variant1,
    #[serde(rename = "2")]
    Variant2,
    #[serde(rename = "3")]
    Variant3,
    #[serde(rename = "4")]
    Variant4,
    #[serde(rename = "5")]
    Variant5,
    #[serde(rename = "last")]
    Last,
}

impl Default for Ordinal {
    fn default() -> Ordinal {
        Self::Variant1
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrdinalValue {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "weekday")]
    Weekday,
    #[serde(rename = "weekend")]
    Weekend,
    #[serde(rename = "sunday")]
    Sunday,
    #[serde(rename = "monday")]
    Monday,
    #[serde(rename = "tuesday")]
    Tuesday,
    #[serde(rename = "wednesday")]
    Wednesday,
    #[serde(rename = "thursday")]
    Thursday,
    #[serde(rename = "friday")]
    Friday,
    #[serde(rename = "saturday")]
    Saturday,
}

impl Default for OrdinalValue {
    fn default() -> OrdinalValue {
        Self::Day
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MonthlyType {
    #[serde(rename = "each")]
    Each,
    #[serde(rename = "on")]
    On,
}

impl Default for MonthlyType {
    fn default() -> MonthlyType {
        Self::Each
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MemberUserDto {
    #[serde(rename = "_id")]
    pub _id: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "email")]
    pub email: String,
}

impl MemberUserDto {
    pub fn new(_id: String, first_name: String, last_name: String, email: String) -> MemberUserDto {
        MemberUserDto {
            _id,
            first_name,
            last_name,
            email,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MemberInviteDto {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "invitationDate")]
    pub invitation_date: String,
    #[serde(rename = "answerDate", skip_serializing_if = "Option::is_none")]
    pub answer_date: Option<String>,
    #[serde(rename = "_inviterId")]
    pub _inviter_id: String,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IPartnerConfigurationResponseDto {
    #[serde(rename = "projectIds", skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<String>>,
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "configurationId")]
    pub configuration_id: String,
    #[serde(rename = "teamId", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    /// Partner Type Enum
    #[serde(rename = "partnerType")]
    pub partner_type: PartnerType,
}
/// Partner Type Enum
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PartnerType {
    #[serde(rename = "vercel")]
    Vercel,
}

impl Default for PartnerType {
    fn default() -> PartnerType {
        Self::Vercel
    }
}
