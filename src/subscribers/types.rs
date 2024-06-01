use crate::shared::{
    ChannelCredentials, ChannelPreference, ChannelSettings, Preference, TemplateResponse,
};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkSubscriberCreateDto {
    pub subscribers: Vec<CreateSubscriberRequestDto>,
}

impl BulkSubscriberCreateDto {
    pub fn new(subscribers: Vec<CreateSubscriberRequestDto>) -> BulkSubscriberCreateDto {
        BulkSubscriberCreateDto { subscribers }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateSubscriberRequestDto {
    /// The internal identifier you used to create this subscriber, usually correlates to the id the user in your systems
    #[serde(rename = "subscriberId")]
    pub subscriber_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// An http url to the profile image of your subscriber
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteSubscriberResponseDto {
    /// A boolean stating the success of the action
    pub acknowledged: bool,
    /// The status enum for the performed action
    pub status: Status,
}

impl DeleteSubscriberResponseDto {
    pub fn new(acknowledged: bool, status: Status) -> DeleteSubscriberResponseDto {
        DeleteSubscriberResponseDto {
            acknowledged,
            status,
        }
    }
}
/// The status enum for the performed action
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "deleted")]
    Deleted,
}

impl Default for Status {
    fn default() -> Status {
        Self::Deleted
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetSubscriberPreferencesResponseDto {
    /// The workflow information and if it is critical or not
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<Box<TemplateResponse>>,
    /// The preferences of the subscriber regarding the related workflow
    pub preference: Box<Preference>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoveSubscribersRequestDto {
    /// List of subscriber identifiers that will be removed to the topic
    pub subscribers: Vec<String>,
}

impl RemoveSubscribersRequestDto {
    pub fn new(subscribers: Vec<String>) -> RemoveSubscribersRequestDto {
        RemoveSubscribersRequestDto { subscribers }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateSubscriberChannelRequestDto {
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
    pub credentials: Box<ChannelCredentials>,
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
pub struct UpdateSubscriberGlobalPreferencesRequestDto {
    /// Enable or disable the subscriber global preferences.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The subscriber global preferences for every ChannelTypeEnum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferences: Option<Vec<ChannelPreference>>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateSubscriberPreferenceRequestDto {
    /// The subscriber preferences for every ChannelTypeEnum for the workflow assigned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Box<ChannelPreference>>,
    /// Sets if the workflow is fully enabled for all channels or not for the subscriber.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateSubscriberPreferenceResponseDto {
    /// The workflow information and if it is critical or not
    pub template: Box<TemplateResponse>,
    /// The preferences of the subscriber regarding the related workflow
    pub preference: Box<Preference>,
}

impl UpdateSubscriberPreferenceResponseDto {
    pub fn new(
        template: TemplateResponse,
        preference: Preference,
    ) -> UpdateSubscriberPreferenceResponseDto {
        UpdateSubscriberPreferenceResponseDto {
            template: Box::new(template),
            preference: Box::new(preference),
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateSubscriberRequestDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriberResponseDto {
    /// The internal id novu generated for your subscriber, this is not the subscriberId matching your query. See `subscriberId` for that
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// The internal identifier you used to create this subscriber, usually correlates to the id the user in your systems
    #[serde(rename = "subscriberId")]
    pub subscriber_id: String,
    /// Channels settings for subscriber
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<ChannelSettings>>,
    #[serde(rename = "isOnline", skip_serializing_if = "Option::is_none")]
    pub is_online: Option<bool>,
    #[serde(rename = "lastOnlineAt", skip_serializing_if = "Option::is_none")]
    pub last_online_at: Option<String>,
    #[serde(rename = "_organizationId")]
    pub _organization_id: String,
    #[serde(rename = "_environmentId")]
    pub _environment_id: String,
    pub deleted: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "__v", skip_serializing_if = "Option::is_none")]
    pub __v: Option<f64>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriberPayloadDto {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// An http url to the profile image of your subscriber
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WithSubscriberId<T> {
    /// The internal identifier you used to create this subscriber, usually correlates to the id the user in your systems
    #[serde(rename = "subscriberId")]
    pub subscriber_id: String,
    #[serde(flatten)]
    inner: T,
}

pub type SubscriberPayloadWithIdDto = WithSubscriberId<SubscriberPayloadDto>;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateSubscriberOnlineFlagRequestDto {
    #[serde(rename = "isOnline")]
    pub is_online: bool,
}

impl UpdateSubscriberOnlineFlagRequestDto {
    pub fn new(is_online: bool) -> UpdateSubscriberOnlineFlagRequestDto {
        UpdateSubscriberOnlineFlagRequestDto { is_online }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnseenCountResponse {
    pub count: f64,
}

impl UnseenCountResponse {
    pub fn new(count: f64) -> UnseenCountResponse {
        UnseenCountResponse { count }
    }
}
