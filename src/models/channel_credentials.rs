/*
 * Novu API
 *
 * Novu REST API. Please see https://docs.novu.co/api-reference for more details.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@novu.co
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

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

impl ChannelCredentials {
    pub fn new(webhook_url: String) -> ChannelCredentials {
        ChannelCredentials {
            webhook_url,
            channel: None,
            device_tokens: None,
            alert_uid: None,
            title: None,
            image_url: None,
            state: None,
            external_url: None,
        }
    }
}

