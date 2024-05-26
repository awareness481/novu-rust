pub mod types;
pub use types::*;

use std::sync::Arc;

use crate::{
    client::{Client, ResponseError},
    shared::DataNumberDto,
};
pub struct Changes {
    client: Arc<Client>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplyOneDto {
    #[serde(rename = "changeId")]
    change_id: String,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetChangesPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    limit: Option<u8>,
    promoted: Option<bool>,
}

impl Changes {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    pub async fn get(&self, data: GetChangesPayload) -> Result<ChangeResponseDto, ResponseError> {
        let mut data = data;

        if data.limit.is_none() {
            data.limit = Some(10)
        }

        if data.promoted.is_none() {
            data.promoted = Some(true);
        }

        match qs::to_string(&data) {
            Ok(query) => self.client.get(format!("/changes?{query}")).await,
            Err(e) => Err(ResponseError::ParseError(e)),
        }
    }

    pub async fn get_count(&self) -> Result<DataNumberDto, ResponseError> {
        self.client.get("/changes/count").await
    }

    pub async fn apply_one(
        &self,
        data: ApplyOneDto,
    ) -> Result<Vec<ChangeResponseDto>, ResponseError> {
        self.client
            .post(format!("changes/{}/apply", data.change_id), None::<&bool>)
            .await
    }

    pub async fn apply_many(
        &self,
        data: BulkApplyChangeDto,
    ) -> Result<Vec<ChangeResponseDto>, ResponseError> {
        self.client.post("/changes/bulk/apply", Some(&data)).await
    }
}
