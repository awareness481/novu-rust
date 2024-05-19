use std::sync::Arc;

use crate::{
    client::{Client, ResponseError},
    models::{CreateNotificationGroupRequestDto, NotificationGroupResponseDto},
};

pub struct NotificationGroups {
    client: Arc<Client>,
}

impl NotificationGroups {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    pub async fn create(
        self,
        data: CreateNotificationGroupRequestDto,
    ) -> Result<NotificationGroupResponseDto, ResponseError> {
        self.client.post("/notification-groups", Some(&data)).await
    }

    pub async fn get(self) -> Result<NotificationGroupResponseDto, ResponseError> {
        self.client.get("/notification-groups").await
    }

    pub async fn get_one(self, id: String) -> Result<NotificationGroupResponseDto, ResponseError> {
        self.client.get(format!("/notification-groups/{id}")).await
    }

    pub async fn update(
        self,
        data: CreateNotificationGroupRequestDto,
    ) -> Result<NotificationGroupResponseDto, ResponseError> {
        self.client.patch("/notification-groups", Some(&data)).await
    }

    pub async fn delete(self, id: String) -> Result<NotificationGroupResponseDto, ResponseError> {
        self.client
            .delete(format!("/notification-groups/{id}"))
            .await
    }
}
