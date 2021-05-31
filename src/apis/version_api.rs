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

use super::configuration;
use crate::errors::errors::CamundaClientError;
pub use reqwest;
pub use serde;
pub use serde_derive;
pub use serde_json;
use std::borrow::Borrow;
use std::sync::Arc;
pub use url;

pub struct VersionApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl VersionApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> VersionApiClient {
        VersionApiClient { configuration }
    }
}

#[async_trait]
pub trait VersionApi {
    async fn get_rest_api_version(&self) -> Result<crate::models::VersionDto, CamundaClientError>;
}

#[async_trait]
impl VersionApi for VersionApiClient {
    async fn get_rest_api_version(&self) -> Result<crate::models::VersionDto, CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/version", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let resp = req_builder.send().await?;

        Ok(resp.error_for_status()?.json().await?)
    }
}
