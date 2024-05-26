pub mod types;
use crate::client::{Client, ResponseError};
use std::sync::Arc;

pub use types::*;

pub struct ExecutationDetails {
    client: Arc<Client>,
}

impl ExecutationDetails {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    pub async fn get(
        &self,
        data: ExecutionDetailsRequestDto,
    ) -> Result<ExecutionDetailsResponseDto, ResponseError> {
        match qs::to_string(&data) {
            Ok(query) => self.client.get(format!("/execution-details?{query}")).await,
            Err(e) => Err(ResponseError::ParseError(e)),
        }
    }
}
