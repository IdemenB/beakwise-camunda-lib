/*
 * Camunda BPM REST API
 *
 * OpenApi Spec for Camunda BPM REST API.
 *
 * The version of the OpenAPI document: 7.13.0
 *
 * Generated by: https://openapi-generator.tech
 */
use async_trait::async_trait;
use std::borrow::Borrow;
use std::option::Option;
use std::sync::Arc;

pub use reqwest;
pub use serde;
pub use serde_derive;
pub use serde_json;
pub use url;

use crate::errors::errors::CamundaClientError;

use super::configuration;

pub struct SignalApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl SignalApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> SignalApiClient {
        SignalApiClient { configuration }
    }
}

#[async_trait]
pub trait SignalApi {
    async fn throw_signal(
        &self,
        signal_dto: Option<crate::models::SignalDto>,
    ) -> Result<(), CamundaClientError>;
}

#[async_trait]
impl SignalApi for SignalApiClient {
    async fn throw_signal(
        &self,
        signal_dto: Option<crate::models::SignalDto>,
    ) -> Result<(), CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/signal", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&signal_dto);

        // send request
        let resp = req_builder.send().await?;

        resp.error_for_status()?.json().await?;
        Ok(())
    }
}
