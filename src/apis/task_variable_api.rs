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

use crate::{errors::errors::WorkflowError, utils::url_encode};
use reqwest;
use std::borrow::Borrow;
use std::option::Option;
use std::sync::Arc;

use super::configuration;

pub struct TaskVariableApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl TaskVariableApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> TaskVariableApiClient {
        TaskVariableApiClient { configuration }
    }
}

#[async_trait]
pub trait TaskVariableApi {
    async fn delete_task_variable(&self, id: &str, var_name: &str) -> Result<(), WorkflowError>;
    async fn get_task_variable(
        &self,
        id: &str,
        var_name: &str,
        deserialize_value: Option<bool>,
    ) -> Result<crate::models::VariableValueDto, WorkflowError>;
    async fn get_task_variable_binary(
        &self,
        id: &str,
        var_name: &str,
    ) -> Result<std::path::PathBuf, WorkflowError>;
    async fn get_task_variables(
        &self,
        id: &str,
        deserialize_value: Option<bool>,
    ) -> Result<::std::collections::HashMap<String, crate::models::VariableValueDto>, WorkflowError>;
    async fn modify_task_variables(
        &self,
        id: &str,
        patch_variables_dto: Option<crate::models::PatchVariablesDto>,
    ) -> Result<(), WorkflowError>;
    async fn put_task_variable(
        &self,
        id: &str,
        var_name: &str,
        variable_value_dto: Option<crate::models::VariableValueDto>,
    ) -> Result<(), WorkflowError>;
    async fn set_binary_task_variable(
        &self,
        id: &str,
        var_name: &str,
        data: Option<std::path::PathBuf>,
        value_type: Option<&str>,
    ) -> Result<(), WorkflowError>;
}

#[async_trait]
impl TaskVariableApi for TaskVariableApiClient {
    async fn delete_task_variable(&self, id: &str, var_name: &str) -> Result<(), WorkflowError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/task/{id}/variables/{varName}",
            configuration.base_path,
            id = url_encode::url_encode(id),
            varName = url_encode::url_encode(var_name)
        );
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let resp = req_builder.send().await?;

        resp.error_for_status()?.json().await?;
        Ok(())
    }

    async fn get_task_variable(
        &self,
        id: &str,
        var_name: &str,
        deserialize_value: Option<bool>,
    ) -> Result<crate::models::VariableValueDto, WorkflowError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/task/{id}/variables/{varName}",
            configuration.base_path,
            id = url_encode::url_encode(id),
            varName = url_encode::url_encode(var_name)
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = deserialize_value {
            req_builder = req_builder.query(&[("deserializeValue", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let resp = req_builder.send().await?;

        Ok(resp.error_for_status()?.json().await?)
    }

    async fn get_task_variable_binary(
        &self,
        id: &str,
        var_name: &str,
    ) -> Result<std::path::PathBuf, WorkflowError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/task/{id}/variables/{varName}/data",
            configuration.base_path,
            id = url_encode::url_encode(id),
            varName = url_encode::url_encode(var_name)
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let resp = req_builder.send().await?;

        Ok(resp.error_for_status()?.json().await?)
    }

    async fn get_task_variables(
        &self,
        id: &str,
        deserialize_value: Option<bool>,
    ) -> Result<::std::collections::HashMap<String, crate::models::VariableValueDto>, WorkflowError>
    {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/task/{id}/variables",
            configuration.base_path,
            id = url_encode::url_encode(id)
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = deserialize_value {
            req_builder = req_builder.query(&[("deserializeValue", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let resp = req_builder.send().await?;

        Ok(resp.error_for_status()?.json().await?)
    }

    async fn modify_task_variables(
        &self,
        id: &str,
        patch_variables_dto: Option<crate::models::PatchVariablesDto>,
    ) -> Result<(), WorkflowError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/task/{id}/variables",
            configuration.base_path,
            id = url_encode::url_encode(id)
        );
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&patch_variables_dto);

        // send request
        let resp = req_builder.send().await?;

        resp.error_for_status()?.json().await?;
        Ok(())
    }

    async fn put_task_variable(
        &self,
        id: &str,
        var_name: &str,
        variable_value_dto: Option<crate::models::VariableValueDto>,
    ) -> Result<(), WorkflowError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/task/{id}/variables/{varName}",
            configuration.base_path,
            id = url_encode::url_encode(id),
            varName = url_encode::url_encode(var_name)
        );
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&variable_value_dto);

        // send request
        let resp = req_builder.send().await?;

        resp.error_for_status()?.json().await?;
        Ok(())
    }

    async fn set_binary_task_variable(
        &self,
        id: &str,
        var_name: &str,
        data: Option<std::path::PathBuf>,
        value_type: Option<&str>,
    ) -> Result<(), WorkflowError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        //async Reqwest currently does not support file. That's why here not a clone of the original client is taken.
        //Instead, a client of blocking type is created from scratch.
        // let client = &configuration.client;
        let client = reqwest::blocking::Client::new();

        let uri_str = format!(
            "{}/task/{id}/variables/{varName}/data",
            configuration.base_path,
            id = url_encode::url_encode(id),
            varName = url_encode::url_encode(var_name)
        );
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        let mut form = reqwest::blocking::multipart::Form::new();
        if let Some(param_value) = data {
            form = form.file("data", param_value)?;
        }
        if let Some(param_value) = value_type {
            form = form.text("valueType", param_value.to_string());
        }
        req_builder = req_builder.multipart(form);

        // send request
        let resp = req_builder.send()?;

        resp.error_for_status()?.json()?;
        Ok(())
    }
}
