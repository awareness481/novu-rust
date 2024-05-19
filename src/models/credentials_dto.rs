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
pub struct CredentialsDto {
    #[serde(rename = "apiKey", skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "secretKey", skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[serde(rename = "secure", skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "accountSid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    #[serde(rename = "messageProfileId", skip_serializing_if = "Option::is_none")]
    pub message_profile_id: Option<String>,
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(rename = "senderName", skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
    #[serde(rename = "projectName", skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[serde(rename = "applicationId", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "requireTls", skip_serializing_if = "Option::is_none")]
    pub require_tls: Option<bool>,
    #[serde(rename = "ignoreTls", skip_serializing_if = "Option::is_none")]
    pub ignore_tls: Option<bool>,
    #[serde(rename = "tlsOptions", skip_serializing_if = "Option::is_none")]
    pub tls_options: Option<serde_json::Value>,
    #[serde(rename = "baseUrl", skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    #[serde(rename = "webhookUrl", skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    #[serde(rename = "redirectUrl", skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(rename = "hmac", skip_serializing_if = "Option::is_none")]
    pub hmac: Option<bool>,
    #[serde(rename = "serviceAccount", skip_serializing_if = "Option::is_none")]
    pub service_account: Option<String>,
    #[serde(rename = "ipPoolName", skip_serializing_if = "Option::is_none")]
    pub ip_pool_name: Option<String>,
    #[serde(
        rename = "apiKeyRequestHeader",
        skip_serializing_if = "Option::is_none"
    )]
    pub api_key_request_header: Option<String>,
    #[serde(
        rename = "secretKeyRequestHeader",
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_key_request_header: Option<String>,
    #[serde(rename = "idPath", skip_serializing_if = "Option::is_none")]
    pub id_path: Option<String>,
    #[serde(rename = "datePath", skip_serializing_if = "Option::is_none")]
    pub date_path: Option<String>,
    #[serde(rename = "apiToken", skip_serializing_if = "Option::is_none")]
    pub api_token: Option<String>,
    #[serde(
        rename = "authenticateByToken",
        skip_serializing_if = "Option::is_none"
    )]
    pub authenticate_by_token: Option<bool>,
    #[serde(
        rename = "authenticationTokenKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub authentication_token_key: Option<String>,
    #[serde(rename = "instanceId", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "alertUid", skip_serializing_if = "Option::is_none")]
    pub alert_uid: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "imageUrl", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "externalLink", skip_serializing_if = "Option::is_none")]
    pub external_link: Option<String>,
    #[serde(rename = "channelId", skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(
        rename = "phoneNumberIdentification",
        skip_serializing_if = "Option::is_none"
    )]
    pub phone_number_identification: Option<String>,
}
