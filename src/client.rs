use reqwest::{Error, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::convert::Into;
use std::fmt::Display;
use std::result::Result;
use thiserror::Error;

pub const NOVU_API_VERSION: &str = "v1";

#[derive(Deserialize, Debug)]
pub struct DataContainer<T> {
    pub data: T,
}

#[derive(Deserialize, Debug)]
pub struct ErrorMessage {
    message: String,
    error: String,
    #[serde(rename = "statusCode")]
    status_code: u32,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Response<T> {
    Success(DataContainer<T>),
    Error(String),
    GeneralError(ErrorMessage),
}

impl<T> Response<T> {
    fn into_result(self) -> Result<T, ResponseError> {
        match self {
            Response::Success(data) => Ok(data.data),
            Response::Error(e) => Err(ResponseError::Error(e)),
            Response::GeneralError(e) => Err(ResponseError::GeneralError(e)),
        }
    }
}

impl Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseError::Error(e) => write!(f, "ApiError: {}", e),
            ResponseError::InternalError(e) => write!(f, "InternalError: {:?}", e),
            ResponseError::ParseError(e) => write!(f, "ParseErrorL {:?}", e),
            ResponseError::GeneralError(e) => write!(f, "GeneralError: {:?}", e),
        }
    }
}

#[derive(Error, Debug)]
pub enum ResponseError {
    Error(String),
    #[error(transparent)]
    InternalError(#[from] reqwest::Error),
    ParseError(#[from] crate::qs::Error),
    GeneralError(ErrorMessage),
}

pub struct Client {
    api_url: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(
        api_key: impl ToString,
        backend_url: Option<impl ToString>,
    ) -> Result<Client, reqwest::Error> {
        Ok(Self {
            api_url: Self::build_backend_url(&backend_url),
            client: Self::build_client(&api_key)?,
        })
    }

    async fn refine_result<T: DeserializeOwned>(
        res: Result<reqwest::Response, Error>,
    ) -> Result<T, ResponseError> {
        match res {
            Ok(response) => {
                let res = response.json::<Response<T>>().await;

                match res {
                    Ok(res) => res.into_result(),
                    Err(err) => Err(ResponseError::InternalError(err)),
                }
            }
            Err(err) => Err(ResponseError::InternalError(err)),
        }
    }

    pub async fn post<T: DeserializeOwned>(
        &self,
        endpoint: impl ToString,
        data: Option<&impl Serialize>,
    ) -> Result<T, ResponseError> {
        let res;

        if let Some(body) = data {
            res = self
                .client
                .post(self.get_url(endpoint))
                .json(body)
                .send()
                .await;
        } else {
            res = self.client.post(self.get_url(endpoint)).send().await;
        }

        Self::refine_result(res).await
    }

    pub async fn get<T: DeserializeOwned>(
        &self,
        endpoint: impl ToString,
    ) -> Result<T, ResponseError> {
        let res = self.client.get(self.get_url(endpoint)).send().await;

        Self::refine_result(res).await
    }

    pub async fn delete<T: DeserializeOwned>(
        &self,
        endpoint: impl ToString,
    ) -> Result<T, ResponseError> {
        let res = self.client.delete(self.get_url(endpoint)).send().await;

        Self::refine_result(res).await
    }

    pub async fn put<T: DeserializeOwned>(
        &self,
        endpoint: impl ToString,
        data: &impl Serialize,
    ) -> Result<T, ResponseError> {
        let res = self
            .client
            .put(self.get_url(endpoint))
            .json(&data)
            .send()
            .await;

        Self::refine_result(res).await
    }

    pub async fn patch<T: DeserializeOwned>(
        &self,
        endpoint: impl ToString,
        data: Option<&impl Serialize>,
    ) -> Result<T, ResponseError> {
        let res = self
            .client
            .patch(self.get_url(endpoint))
            .json(&data)
            .send()
            .await;

        Self::refine_result(res).await
    }

    fn build_backend_url(backend_url: &Option<impl ToString>) -> String {
        if let Some(backend_url) = backend_url {
            let backend_url = &backend_url.to_string();

            if backend_url.contains("novu.co/v") {
                return backend_url.to_string();
            }

            return format!("{}/{}", backend_url, NOVU_API_VERSION);
        }

        format!("https://api.novu.co/{}", NOVU_API_VERSION)
    }

    fn build_client(api_key: &impl ToString) -> Result<reqwest::Client, Error> {
        let mut default_headers = reqwest::header::HeaderMap::new();

        default_headers.insert(
            "Authorization",
            reqwest::header::HeaderValue::from_str(&format!("ApiKey {}", &api_key.to_string()))
                .unwrap(),
        );

        reqwest::Client::builder()
            .default_headers(default_headers)
            .build()
    }

    fn get_url(&self, endpoint: impl ToString) -> String {
        format!("{}{}", self.api_url, endpoint.to_string())
    }
}
