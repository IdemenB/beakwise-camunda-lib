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

pub struct TaskAttachmentApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl TaskAttachmentApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> TaskAttachmentApiClient {
        TaskAttachmentApiClient { configuration }
    }
}

pub trait TaskAttachmentApi {
    fn add_attachment(
        &self,
        id: &str,
        attachment_name: Option<&str>,
        attachment_description: Option<&str>,
        attachment_type: Option<&str>,
        url: Option<&str>,
        content: Option<std::path::PathBuf>,
    ) -> Result<crate::models::AttachmentDto, CamundaClientError>;
    fn delete_attachment(&self, id: &str, attachment_id: &str) -> Result<(), CamundaClientError>;
    fn get_attachment(
        &self,
        id: &str,
        attachment_id: &str,
    ) -> Result<crate::models::AttachmentDto, CamundaClientError>;
    fn get_attachment_data(
        &self,
        id: &str,
        attachment_id: &str,
    ) -> Result<std::path::PathBuf, CamundaClientError>;
    fn get_attachments(
        &self,
        id: &str,
    ) -> Result<Vec<crate::models::AttachmentDto>, CamundaClientError>;
}

impl TaskAttachmentApi for TaskAttachmentApiClient {
    fn add_attachment(
        &self,
        id: &str,
        attachment_name: Option<&str>,
        attachment_description: Option<&str>,
        attachment_type: Option<&str>,
        url: Option<&str>,
        content: Option<std::path::PathBuf>,
    ) -> Result<crate::models::AttachmentDto, CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/task/{id}/attachment/create",
            configuration.base_path,
            id = url_encode::url_encode(id)
        );
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        let mut form = reqwest::blocking::multipart::Form::new();
        if let Some(param_value) = attachment_name {
            form = form.text("attachment-name", param_value.to_string());
        }
        if let Some(param_value) = attachment_description {
            form = form.text("attachment-description", param_value.to_string());
        }
        if let Some(param_value) = attachment_type {
            form = form.text("attachment-type", param_value.to_string());
        }
        if let Some(param_value) = url {
            form = form.text("url", param_value.to_string());
        }
        if let Some(param_value) = content {
            form = form.file("content", param_value)?;
        }
        req_builder = req_builder.multipart(form);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn delete_attachment(&self, id: &str, attachment_id: &str) -> Result<(), CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/task/{id}/attachment/{attachmentId}",
            configuration.base_path,
            id = url_encode::url_encode(id),
            attachmentId = url_encode::url_encode(attachment_id)
        );
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn get_attachment(
        &self,
        id: &str,
        attachment_id: &str,
    ) -> Result<crate::models::AttachmentDto, CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/task/{id}/attachment/{attachmentId}",
            configuration.base_path,
            id = url_encode::url_encode(id),
            attachmentId = url_encode::url_encode(attachment_id)
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_attachment_data(
        &self,
        id: &str,
        attachment_id: &str,
    ) -> Result<std::path::PathBuf, CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/task/{id}/attachment/{attachmentId}/data",
            configuration.base_path,
            id = url_encode::url_encode(id),
            attachmentId = url_encode::url_encode(attachment_id)
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_attachments(
        &self,
        id: &str,
    ) -> Result<Vec<crate::models::AttachmentDto>, CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/task/{id}/attachment",
            configuration.base_path,
            id = url_encode::url_encode(id)
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }
}
