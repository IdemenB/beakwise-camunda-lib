/*
 * Camunda BPM REST API
 *
 * OpenApi Spec for Camunda BPM REST API.
 *
 * The version of the OpenAPI document: 7.13.0
 *
 * Generated by: https://openapi-generator.tech
 */

use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;
use std::sync::Arc;

use crate::{errors::errors::CamundaClientError, utils::url_encode};
use reqwest;

use super::configuration;

pub struct EngineApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl EngineApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> EngineApiClient {
        EngineApiClient { configuration }
    }
}

pub trait EngineApi {
    fn get_process_engine_names(
        &self,
    ) -> Result<Vec<crate::models::ProcessEngineDto>, CamundaClientError>;
}

impl EngineApi for EngineApiClient {
    fn get_process_engine_names(
        &self,
    ) -> Result<Vec<crate::models::ProcessEngineDto>, CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/engine", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }
}
