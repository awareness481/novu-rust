use crate::client::{Client, ResponseError};
use std::sync::Arc;

pub use types::*;

pub mod types;

pub struct InboundParse {
    client: Arc<Client>,
}

impl InboundParse {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    pub async fn get_mx_status(&self) -> Result<Option<GetMxRecordResponseDto>, ResponseError> {
        self.client.get("/inbound-parse/mx/status").await
    }
}
