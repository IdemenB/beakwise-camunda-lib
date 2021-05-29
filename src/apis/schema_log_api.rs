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
use std::rc::Rc;

use crate::errors::errors::WorkflowError;

use super::configuration;
pub use reqwest;
pub use serde;
pub use serde_derive;
pub use serde_json;
pub use url;

pub struct SchemaLogApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl SchemaLogApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> SchemaLogApiClient {
        SchemaLogApiClient { configuration }
    }
}

pub trait SchemaLogApi {
    fn get_schema_log(
        &self,
        version: Option<&str>,
        first_result: Option<i32>,
        max_results: Option<i32>,
    ) -> Result<Vec<crate::models::SchemaLogEntryDto>, WorkflowError>;
    fn query_schema_log(
        &self,
        first_result: Option<i32>,
        max_results: Option<i32>,
        schema_log_query_dto: Option<crate::models::SchemaLogQueryDto>,
    ) -> Result<Vec<crate::models::SchemaLogEntryDto>, WorkflowError>;
}

impl SchemaLogApi for SchemaLogApiClient {
    fn get_schema_log(
        &self,
        version: Option<&str>,
        first_result: Option<i32>,
        max_results: Option<i32>,
    ) -> Result<Vec<crate::models::SchemaLogEntryDto>, WorkflowError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/schema/log", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = version {
            req_builder = req_builder.query(&[("version", &s.to_string())]);
        }
        if let Some(ref s) = first_result {
            req_builder = req_builder.query(&[("firstResult", &s.to_string())]);
        }
        if let Some(ref s) = max_results {
            req_builder = req_builder.query(&[("maxResults", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn query_schema_log(
        &self,
        first_result: Option<i32>,
        max_results: Option<i32>,
        schema_log_query_dto: Option<crate::models::SchemaLogQueryDto>,
    ) -> Result<Vec<crate::models::SchemaLogEntryDto>, WorkflowError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/schema/log", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref s) = first_result {
            req_builder = req_builder.query(&[("firstResult", &s.to_string())]);
        }
        if let Some(ref s) = max_results {
            req_builder = req_builder.query(&[("maxResults", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&schema_log_query_dto);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }
}
