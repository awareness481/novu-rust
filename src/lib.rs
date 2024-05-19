#![allow(unused_imports)]

use std::sync::Arc;

use changes::Changes;
use client::{Client, Response, ResponseError};
use environments::Environments;
use executation_details::ExecutationDetails;
use feeds::Feeds;
use inbound_parse::InboundParse;
use integrations::Integrations;
use layouts::Layouts;
use messages::Messages;
use models::{
    BulkTriggerEventDto, SubscriberPayloadDto, TriggerEventRequestDto, TriggerEventRequestDtoActor,
    TriggerEventResponseDto, TriggerEventToAllRequestDto,
};
use notification_groups::NotificationGroups;
use organizations::Organizations;
use reqwest::Error;
use subscribers::Subscribers;
use tenants::Tenants;
use topics::Topics;
use workflow_override::WorkflowOverride;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_qs as qs;
// extern crate url;
extern crate reqwest;

pub mod changes;
pub mod client;
pub mod environments;
pub mod executation_details;
pub mod feeds;
pub mod inbound_parse;
pub mod integrations;
pub mod layouts;
pub mod messages;
pub mod models;
pub mod notification_groups;
pub mod organizations;
pub mod subscribers;
pub mod tenants;
pub mod topics;
pub mod workflow_override;

pub struct Novu {
    client: Arc<Client>,
    environments: Environments,
    feeds: Feeds,
    layouts: Layouts,
    changes: Changes,
    inbound_parse: InboundParse,
    executation_details: ExecutationDetails,
    integrations: Integrations,
    messages: Messages,
    notification_groupss: NotificationGroups,
    organizations: Organizations,
    subscribers: Subscribers,
    tenants: Tenants,
    topics: Topics,
    workflow_override: WorkflowOverride,
}

#[derive(Serialize)]
pub struct TriggerPayload {
    pub name: String,
    pub to: SubscriberPayloadDto,
}

impl Novu {
    pub fn new(api_key: impl ToString, api_url: Option<&str>) -> Result<Self, Error> {
        let client = Client::new(api_key, api_url)?;
        let client = Arc::new(client);

        Ok(Self {
            client: client.clone(),
            environments: Environments::new(client.clone()),
            feeds: Feeds::new(client.clone()),
            layouts: Layouts::new(client.clone()),
            changes: Changes::new(client.clone()),
            inbound_parse: InboundParse::new(client.clone()),
            executation_details: ExecutationDetails::new(client.clone()),
            integrations: Integrations::new(client.clone()),
            messages: Messages::new(client.clone()),
            notification_groupss: NotificationGroups::new(client.clone()),
            organizations: Organizations::new(client.clone()),
            subscribers: Subscribers::new(client.clone()),
            tenants: Tenants::new(client.clone()),
            topics: Topics::new(client.clone()),
            workflow_override: WorkflowOverride::new(client.clone()),
        })
    }

    pub async fn trigger(
        self,
        data: TriggerPayload,
    ) -> Result<TriggerEventResponseDto, ResponseError> {
        self.client.post("/events/trigger", Some(&data)).await
    }

    pub async fn bulk_trigger(
        self,
        events: BulkTriggerEventDto,
    ) -> Result<BulkTriggerEventDto, ResponseError> {
        self.client
            .post("/events/trigger/bulk", Some(&events))
            .await
    }

    pub async fn broadcast(
        self,
        data: TriggerEventToAllRequestDto,
    ) -> Result<TriggerEventResponseDto, ResponseError> {
        self.client
            .post("/events/trigger/broadcast", Some(&data))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
